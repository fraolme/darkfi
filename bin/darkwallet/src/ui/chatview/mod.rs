/* This file is part of DarkFi (https://dark.fi)
 *
 * Copyright (C) 2020-2024 Dyne.org foundation
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use async_lock::Mutex as AsyncMutex;
use atomic_float::AtomicF32;
use chrono::{Local, TimeZone};
use darkfi::system::{msleep, CondVar};
use darkfi_serial::{
    async_trait, deserialize, Decodable, Encodable, SerialDecodable, SerialEncodable,
};
use miniquad::{KeyCode, KeyMods, TouchPhase};
use rand::{rngs::OsRng, Rng};
use sled_overlay::sled;
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    io::Cursor,
    sync::{atomic::Ordering, Arc, Mutex as SyncMutex, Weak},
};

mod page;
use page::{FreedData, MessageBuffer};

use crate::{
    gfx::{
        DrawCall, DrawInstruction, DrawMesh, GraphicsEventPublisherPtr, Point, Rectangle,
        RenderApi, RenderApiPtr,
    },
    mesh::{Color, MeshBuilder, COLOR_BLUE, COLOR_GREEN},
    prop::{PropertyBool, PropertyColor, PropertyFloat32, PropertyPtr, PropertyUint32, Role},
    pubsub::Subscription,
    scene::{Pimpl, SceneGraph, SceneGraphPtr2, SceneNodeId},
    text::{self, Glyph, GlyphPositionIter, TextShaperPtr},
    util::enumerate,
    ExecutorPtr,
};

use super::{eval_rect, get_parent_rect, read_rect, DrawUpdate, OnModify};

const EPSILON: f32 = 0.001;
const BIG_EPSILON: f32 = 0.05;

fn is_whitespace(s: &str) -> bool {
    s.chars().all(char::is_whitespace)
}

fn is_zero(x: f32) -> bool {
    x.abs() < EPSILON
}

/// std::cmp::max() doesn't work on f32
fn max(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

#[derive(Clone, Debug, SerialEncodable, SerialDecodable)]
pub struct ChatMsg {
    pub nick: String,
    pub text: String,
}

type Timestamp = u64;
type MessageId = [u8; 32];

const PAGE_SIZE: usize = 10;
const PRELOAD_PAGES: usize = 10;

#[derive(Clone)]
struct TouchInfo {
    start_scroll: f32,
    start_y: f32,
    start_instant: std::time::Instant,
    last_y: f32,
}

impl TouchInfo {
    fn new() -> Self {
        Self { start_scroll: 0., start_y: 0., start_instant: std::time::Instant::now(), last_y: 0. }
    }
}

pub type ChatViewPtr = Arc<ChatView>;

pub struct ChatView {
    node_id: SceneNodeId,
    #[allow(dead_code)]
    tasks: Vec<smol::Task<()>>,
    sg: SceneGraphPtr2,
    render_api: RenderApiPtr,
    text_shaper: TextShaperPtr,
    tree: sled::Tree,

    msgbuf: AsyncMutex<MessageBuffer>,
    dc_key: u64,

    /// Used for detecting when scrolling view
    mouse_pos: SyncMutex<Point>,
    /// Touch scrolling
    touch_info: SyncMutex<TouchInfo>,

    rect: PropertyPtr,
    scroll: PropertyFloat32,
    font_size: PropertyFloat32,
    line_height: PropertyFloat32,
    baseline: PropertyFloat32,
    timestamp_color: PropertyColor,
    text_color: PropertyColor,
    nick_colors: PropertyPtr,
    z_index: PropertyUint32,
    debug: PropertyBool,

    mouse_scroll_start_accel: PropertyFloat32,
    mouse_scroll_decel: PropertyFloat32,
    mouse_scroll_resist: PropertyFloat32,

    // Scroll accel
    motion_cv: Arc<CondVar>,
    accel: AtomicF32,
    speed: AtomicF32,

    /// Used for correct converting input event pos from screen to widget space.
    /// We also use it when we re-eval rect when its changed via property.
    parent_rect: SyncMutex<Option<Rectangle>>,
}

impl ChatView {
    pub async fn new(
        ex: ExecutorPtr,
        sg: SceneGraphPtr2,
        node_id: SceneNodeId,
        render_api: RenderApiPtr,
        event_pub: GraphicsEventPublisherPtr,
        text_shaper: TextShaperPtr,
        tree: sled::Tree,
        recvr: async_channel::Receiver<Vec<u8>>,
    ) -> Pimpl {
        debug!(target: "ui::chatview", "ChatView::new()");
        let scene_graph = sg.lock().await;
        let node = scene_graph.get_node(node_id).unwrap();
        let node_name = node.name.clone();

        let rect = node.get_property("rect").expect("ChatView::rect");
        let scroll = PropertyFloat32::wrap(node, Role::Internal, "scroll", 0).unwrap();
        let font_size = PropertyFloat32::wrap(node, Role::Internal, "font_size", 0).unwrap();
        let line_height = PropertyFloat32::wrap(node, Role::Internal, "line_height", 0).unwrap();
        let baseline = PropertyFloat32::wrap(node, Role::Internal, "baseline", 0).unwrap();
        let timestamp_color = PropertyColor::wrap(node, Role::Internal, "timestamp_color").unwrap();
        let text_color = PropertyColor::wrap(node, Role::Internal, "text_color").unwrap();
        let nick_colors = node.get_property("nick_colors").expect("ChatView::nick_colors");
        let z_index = PropertyUint32::wrap(node, Role::Internal, "z_index", 0).unwrap();
        let debug = PropertyBool::wrap(node, Role::Internal, "debug", 0).unwrap();

        let mouse_scroll_start_accel =
            PropertyFloat32::wrap(node, Role::Internal, "mouse_scroll_start_accel", 0).unwrap();
        let mouse_scroll_decel =
            PropertyFloat32::wrap(node, Role::Internal, "mouse_scroll_decel", 0).unwrap();
        let mouse_scroll_resist =
            PropertyFloat32::wrap(node, Role::Internal, "mouse_scroll_resist", 0).unwrap();
        drop(scene_graph);

        let self_ = Arc::new_cyclic(|me: &Weak<Self>| {
            let ev_sub = event_pub.subscribe_mouse_wheel();
            let me2 = me.clone();
            let mouse_wheel_task =
                ex.spawn(async move { while Self::process_mouse_wheel(&me2, &ev_sub).await {} });

            let ev_sub = event_pub.subscribe_mouse_move();
            let me2 = me.clone();
            let mouse_move_task =
                ex.spawn(async move { while Self::process_mouse_move(&me2, &ev_sub).await {} });

            let ev_sub = event_pub.subscribe_touch();
            let me2 = me.clone();
            let touch_task =
                ex.spawn(async move { while Self::process_touch(&me2, &ev_sub).await {} });

            let ev_sub = event_pub.subscribe_key_down();
            let me2 = me.clone();
            let key_down_task =
                ex.spawn(async move { while Self::process_key_down(&me2, &ev_sub).await {} });

            let me2 = me.clone();
            let insert_line_method_task =
                ex.spawn(
                    async move { while Self::process_insert_line_method(&me2, &recvr).await {} },
                );

            let me2 = me.clone();
            let motion_cv = Arc::new(CondVar::new());
            let cv = motion_cv.clone();
            let motion_task = ex.spawn(async move {
                loop {
                    cv.wait().await;
                    let Some(self_) = me2.upgrade() else {
                        // Should not happen
                        panic!("self destroyed before motion_task was stopped!");
                    };
                    self_.handle_movement().await;
                    cv.reset();
                }
            });

            let mut on_modify = OnModify::new(ex, node_name, node_id, me.clone());

            async fn reload_view(self_: Arc<ChatView>) {
                self_.scrollview(self_.scroll.get()).await;
            }
            on_modify.when_change(scroll.prop(), reload_view);

            //async fn redraw(self_: Arc<ChatView>) {
            //    self_.redraw().await;
            //}
            //on_modify.when_change(rect.clone(), redraw);
            //on_modify.when_change(debug.prop(), redraw);

            let mut tasks = vec![
                mouse_wheel_task,
                mouse_move_task,
                touch_task,
                key_down_task,
                insert_line_method_task,
                motion_task,
            ];
            tasks.append(&mut on_modify.tasks);

            Self {
                node_id,
                tasks,
                sg,
                render_api: render_api.clone(),
                text_shaper: text_shaper.clone(),
                tree,

                msgbuf: AsyncMutex::new(MessageBuffer::new(render_api, text_shaper)),
                dc_key: OsRng.gen(),

                mouse_pos: SyncMutex::new(Point::from([0., 0.])),
                touch_info: SyncMutex::new(TouchInfo::new()),

                rect,
                scroll,
                font_size,
                line_height,
                baseline,
                timestamp_color,
                text_color,
                nick_colors,
                z_index,
                debug,

                mouse_scroll_start_accel,
                mouse_scroll_decel,
                mouse_scroll_resist,

                motion_cv,
                accel: AtomicF32::new(0.),
                speed: AtomicF32::new(0.),

                parent_rect: SyncMutex::new(None),
            }
        });
        Pimpl::ChatView(self_)
    }

    async fn process_mouse_wheel(me: &Weak<Self>, ev_sub: &Subscription<(f32, f32)>) -> bool {
        let Ok((wheel_x, wheel_y)) = ev_sub.receive().await else {
            debug!(target: "ui::chatview", "Event relayer closed");
            return false
        };

        let Some(self_) = me.upgrade() else {
            // Should not happen
            panic!("self destroyed before mouse_wheel_task was stopped!");
        };

        self_.handle_mouse_wheel(wheel_x, wheel_y).await;
        true
    }

    async fn process_mouse_move(me: &Weak<Self>, ev_sub: &Subscription<(f32, f32)>) -> bool {
        let Ok((mouse_x, mouse_y)) = ev_sub.receive().await else {
            debug!(target: "ui::chatview", "Event relayer closed");
            return false
        };

        let Some(self_) = me.upgrade() else {
            // Should not happen
            panic!("self destroyed before mouse_move_task was stopped!");
        };

        self_.handle_mouse_move(mouse_x, mouse_y).await;
        true
    }

    async fn process_touch(
        me: &Weak<Self>,
        ev_sub: &Subscription<(TouchPhase, u64, f32, f32)>,
    ) -> bool {
        let Ok((phase, id, touch_x, touch_y)) = ev_sub.receive().await else {
            debug!(target: "ui::chatview", "Event relayer closed");
            return false
        };

        let Some(self_) = me.upgrade() else {
            // Should not happen
            panic!("self destroyed before touch_task was stopped!");
        };

        self_.handle_touch(phase, id, touch_x, touch_y).await;
        true
    }

    async fn process_key_down(
        me: &Weak<Self>,
        ev_sub: &Subscription<(KeyCode, KeyMods, bool)>,
    ) -> bool {
        let Ok((key, _mods, repeat)) = ev_sub.receive().await else {
            debug!(target: "ui::editbox", "Event relayer closed");
            return false
        };

        if repeat {
            return true
        }

        let Some(self_) = me.upgrade() else {
            // Should not happen
            panic!("self destroyed before char_task was stopped!");
        };

        match key {
            KeyCode::PageUp => {
                let scroll = self_.scroll.get() + 200.;
                self_.scrollview(scroll).await;
            }
            KeyCode::PageDown => {
                let scroll = self_.scroll.get() - 200.;
                self_.scrollview(scroll).await;
            }
            _ => {}
        }

        true
    }

    async fn process_insert_line_method(
        me: &Weak<Self>,
        recvr: &async_channel::Receiver<Vec<u8>>,
    ) -> bool {
        let Ok(data) = recvr.recv().await else {
            debug!(target: "ui::chatview", "Event relayer closed");
            return false
        };

        fn decode_data(data: &[u8]) -> std::io::Result<(Timestamp, MessageId, String, String)> {
            let mut cur = Cursor::new(&data);
            let timestamp = Timestamp::decode(&mut cur)?;
            let message_id = MessageId::decode(&mut cur)?;
            let nick = String::decode(&mut cur)?;
            let text = String::decode(&mut cur)?;
            Ok((timestamp, message_id, nick, text))
        }

        let Ok((timestamp, message_id, nick, text)) = decode_data(&data) else {
            error!(target: "ui::chatview", "insert_line() method invalid arg data");
            return true
        };

        let Some(self_) = me.upgrade() else {
            // Should not happen
            panic!("self destroyed before touch_task was stopped!");
        };

        self_.handle_insert_line(timestamp, message_id, nick, text).await;
        true
    }

    async fn handle_mouse_wheel(&self, wheel_x: f32, wheel_y: f32) {
        //debug!(target: "ui::chatview", "handle_mouse_wheel({wheel_x}, {wheel_y})");

        let Some(rect) = self.parent_rect.lock().unwrap().clone() else { return };

        let mouse_pos = self.mouse_pos.lock().unwrap().clone();
        if !rect.contains(&mouse_pos) {
            //debug!(target: "ui::chatview", "not inside rect");
            return
        }

        self.accel.fetch_add(wheel_y * self.mouse_scroll_start_accel.get(), Ordering::Relaxed);
        self.motion_cv.notify();
    }

    async fn handle_mouse_move(&self, mouse_x: f32, mouse_y: f32) {
        //debug!(target: "ui::chatview", "handle_mouse_move({mouse_x}, {mouse_y})");
        let mut mouse_pos = self.mouse_pos.lock().unwrap();
        mouse_pos.x = mouse_x;
        mouse_pos.y = mouse_y;
    }

    async fn handle_touch(&self, phase: TouchPhase, id: u64, touch_x: f32, touch_y: f32) {
        // Ignore multi-touch
        if id != 0 {
            return
        }

        let Some(rect) = self.parent_rect.lock().unwrap().clone() else { return };

        let touch_pos = Point { x: touch_x, y: touch_y };
        if !rect.contains(&touch_pos) {
            //debug!(target: "ui::chatview", "not inside rect");
            return
        }

        // Simulate mouse events
        match phase {
            TouchPhase::Started => {
                let mut touch_info = self.touch_info.lock().unwrap();
                touch_info.start_scroll = self.scroll.get();
                touch_info.start_y = touch_y;
                touch_info.start_instant = std::time::Instant::now();
                touch_info.last_y = touch_y;
            }
            TouchPhase::Moved => {
                let (start_scroll, start_y) = {
                    let mut touch_info = self.touch_info.lock().unwrap();
                    touch_info.last_y = touch_y;
                    (touch_info.start_scroll, touch_info.start_y)
                };

                let dist = touch_y - start_y;
                // TODO the line selected should be fixed and move exactly that distance
                // No use of multipliers
                // TODO we are maybe doing too many updates so make a widget to 'slow down'
                // how often we move to fixed intervals.
                // draw a poly shape and eval each line segment.
                let scroll = start_scroll + dist;
                self.scrollview(scroll).await;
            }
            TouchPhase::Ended => {
                // Now calculate scroll acceleration
                let touch_info = self.touch_info.lock().unwrap().clone();

                let time = touch_info.start_instant.elapsed().as_millis_f32();
                let dist = touch_y - touch_info.start_y;

                let accel = self.mouse_scroll_start_accel.get() * dist / time;
                self.accel.fetch_add(accel, Ordering::Relaxed);
                self.motion_cv.notify();
            }
            TouchPhase::Cancelled => {}
        }
    }

    async fn add_line_to_db(
        &self,
        timest: Timestamp,
        message_id: &MessageId,
        nick: &str,
        text: &str,
    ) -> bool {
        let timest = timest.to_be_bytes();
        assert_eq!(timest.len(), 8);
        let mut key = [0u8; 8 + 32];
        key[..8].clone_from_slice(&timest);
        key[8..].clone_from_slice(message_id);

        // When does this return Err?
        let contains_key = self.tree.contains_key(&key);
        if contains_key.is_err() || contains_key.unwrap() {
            // Already exists
            return false
        }

        let msg = ChatMsg { nick: nick.to_string(), text: text.to_string() };
        let mut val = vec![];
        msg.encode(&mut val).unwrap();

        self.tree.insert(&key, val).unwrap();
        let _ = self.tree.flush_async().await;
        true
    }
    async fn handle_insert_line(
        &self,
        timest: Timestamp,
        message_id: MessageId,
        nick: String,
        text: String,
    ) {
        debug!(target: "ui::chatview", "handle_insert_line({timest}, {message_id:?}, {nick}, {text})");

        if !self.add_line_to_db(timest, &message_id, &nick, &text).await {
            // Already exists so bail
            debug!(target: "ui::chatview", "duplicate msg so bailing");
            return
        }

        // Add message to page
        self.msgbuf
            .lock()
            .await
            .insert_privmsg(self.font_size.get(), timest, message_id, nick.clone(), text.clone())
            .await;

        // This will refresh the view, so we just use this
        let scroll = self.scroll.get();
        self.scrollview(scroll).await;
    }

    async fn handle_movement(&self) {
        loop {
            msleep(20).await;
            let mut accel = self.accel.load(Ordering::Relaxed);
            let mut speed = self.speed.fetch_add(accel, Ordering::Relaxed) + accel;
            accel *= self.mouse_scroll_decel.get();
            if accel.abs() < 0.05 {
                accel = 0.;
            }
            self.accel.store(accel, Ordering::Relaxed);

            // Apply constant decel to speed
            if is_zero(accel) {
                speed *= self.mouse_scroll_resist.get();
                if speed.abs() < BIG_EPSILON {
                    speed = 0.;
                }
                self.speed.store(speed, Ordering::Relaxed);
            }

            // Finished
            if is_zero(accel) && is_zero(speed) {
                return
            }

            if is_zero(speed) {
                self.accel.store(0., Ordering::Relaxed);
                self.speed.store(0., Ordering::Relaxed);
                return
            }

            let scroll = self.scroll.get() + speed;
            let dist = self.scrollview(scroll).await;

            if is_zero(dist) {
                self.accel.store(0., Ordering::Relaxed);
                self.speed.store(0., Ordering::Relaxed);
                return
            }
        }
    }

    /// Descent = line height - baseline
    fn descent(&self) -> f32 {
        self.line_height.get() - self.baseline.get()
    }

    /// Load extra msgs
    async fn preload_msgs(&self, msgbuf: &mut MessageBuffer) -> usize {
        // Get the current earliest timestamp
        let iter = match msgbuf.oldest_timestamp() {
            Some(oldest_timest) => {
                // iterate from there
                debug!(target: "ui::chatview", "preloading from {oldest_timest}");
                let timest = (oldest_timest - 1).to_be_bytes();
                let mut key = [0u8; 8 + 32];
                key[..8].clone_from_slice(&timest);

                let iter = self.tree.range(..key).rev();
                iter
            }
            None => self.tree.iter().rev(),
        };

        self.load_n_msgs(msgbuf, iter, PRELOAD_PAGES).await
    }

    async fn load_n_msgs<I: Iterator<Item = sled::Result<(sled::IVec, sled::IVec)>>>(
        &self,
        msgbuf: &mut MessageBuffer,
        iter: I,
        n: usize,
    ) -> usize {
        let mut msgs_len = 0;

        for entry in iter {
            let Ok((k, v)) = entry else { break };
            assert_eq!(k.len(), 8 + 32);
            let timest_bytes: [u8; 8] = k[..8].try_into().unwrap();
            let message_id: MessageId = k[8..].try_into().unwrap();
            let timest = Timestamp::from_be_bytes(timest_bytes);
            let chatmsg: ChatMsg = deserialize(&v).unwrap();
            debug!(target: "ui::chatview", "{timest:?} {chatmsg:?}");

            msgbuf
                .push_privmsg(self.font_size.get(), timest, message_id, chatmsg.nick, chatmsg.text)
                .await;

            msgs_len += 1;
        }
        debug!(target: "ui::chatview", "populated {} messages", msgs_len);
        msgs_len
    }

    fn read_nick_colors(&self) -> Vec<Color> {
        let mut colors = vec![];
        let mut color = [0f32; 4];
        for i in 0..self.nick_colors.get_len() {
            color[i % 4] = self.nick_colors.get_f32(i).expect("prop logic err");

            if i > 0 && i % 4 == 0 {
                let color = std::mem::take(&mut color);
                colors.push(color);
            }
        }
        colors
    }

    /// Invalidates cache and redraws everything
    async fn redraw(&self) {
        debug!(target: "ui::chatview", "redraw()");
        let sg = self.sg.lock().await;
        let node = sg.get_node(self.node_id).unwrap();

        let Some(parent_rect) = get_parent_rect(&sg, node) else {
            return;
        };

        let Some(draw_update) = self.draw(&sg, &parent_rect).await else {
            error!(target: "ui::chatview", "ChatView {:?} failed to draw", node);
            return;
        };
        self.render_api.replace_draw_calls(draw_update.draw_calls).await;
        debug!(target: "ui::chatview", "replace draw calls done");
        for buffer_id in draw_update.freed_buffers {
            self.render_api.delete_buffer(buffer_id);
        }
        for texture_id in draw_update.freed_textures {
            self.render_api.delete_texture(texture_id);
        }
    }

    /// Basically a version of redraw() which doesn't invalidate the cache
    async fn scrollview(&self, mut scroll: f32) -> f32 {
        //debug!(target: "ui::chatview", "scrollview()");
        let old_scroll = self.scroll.get();

        let rect = read_rect(self.rect.clone()).expect("bad rect property");

        let mut msgbuf = self.msgbuf.lock().await;

        if let Some(new_scroll) = self.adjust_scroll(&mut msgbuf, scroll, rect.h).await {
            scroll = new_scroll;
        }

        let (mut mesh_instrs, freed) = self.get_meshes(&mut msgbuf, &rect).await;
        drop(msgbuf);

        let mut instrs = vec![DrawInstruction::ApplyViewport(rect)];
        instrs.append(&mut mesh_instrs);

        let draw_calls =
            vec![(self.dc_key, DrawCall { instrs, dcs: vec![], z_index: self.z_index.get() })];

        self.render_api.replace_draw_calls(draw_calls).await;

        for buffer_id in freed.buffers {
            self.render_api.delete_buffer(buffer_id);
        }
        for texture_id in freed.textures {
            self.render_api.delete_texture(texture_id);
        }

        self.scroll.set(scroll);
        scroll - old_scroll
    }

    /// Adjusts a proposed scroll value to clamp it within range. It will load pages until we
    /// either run out or we have enough, then checks scroll is within range.
    /// Returns None if the value is within range.
    async fn adjust_scroll(
        &self,
        msgbuf: &mut MessageBuffer,
        mut scroll: f32,
        rect_h: f32,
    ) -> Option<f32> {
        let line_height = self.line_height.get();
        let baseline = self.baseline.get();

        // We still wish to preload pages to fill the screen, so we just adjust it up to 0.
        let nonneg_scroll = max(scroll, 0.);

        let mut total_height = msgbuf.calc_total_height(line_height, baseline);
        // Load pages until we run out or we have enough
        while total_height < nonneg_scroll + rect_h {
            let n_loaded_pages = self.preload_msgs(msgbuf).await;
            debug!(target: "ui::chatview", "set_adjusted_scroll() loaded until {:?}", msgbuf.oldest_timestamp());

            // We need this value after so first update it
            total_height = msgbuf.calc_total_height(line_height, baseline);

            // No more pages available to load
            if n_loaded_pages == 0 {
                break
            }
        }

        if scroll < 0. {
            return Some(0.)
        }

        let max_allowed_scroll = if total_height > rect_h { total_height - rect_h } else { 0. };

        if scroll > max_allowed_scroll {
            scroll = max_allowed_scroll;
            assert!(scroll >= 0.);
            return Some(scroll)
        }

        // Unchanged
        None
    }

    /// Returns draw calls for drawing
    async fn get_meshes(
        &self,
        msgbuf: &mut MessageBuffer,
        rect: &Rectangle,
    ) -> (Vec<DrawInstruction>, FreedData) {
        let scroll = self.scroll.get();
        let font_size = self.font_size.get();
        let line_height = self.line_height.get();
        let baseline = self.baseline.get();
        let debug_render = self.debug.get();

        let total_height = msgbuf.calc_total_height(line_height, baseline);
        // If lines aren't enough to fill the available buffer then start from the top
        let start_pos = if total_height < rect.h { total_height } else { rect.h };

        let mut instrs = vec![];
        //let mut old_drawmesh = vec![];

        let timest_color = self.timestamp_color.get();
        let text_color = self.text_color.get();
        let nick_colors = self.read_nick_colors();

        let meshes = msgbuf
            .gen_meshes(
                rect,
                scroll,
                font_size,
                line_height,
                baseline,
                &nick_colors,
                timest_color.clone(),
                text_color.clone(),
                debug_render,
            )
            .await;

        let mut current_height = 0.;
        for (i, (y_pos, mesh)) in enumerate(meshes) {
            // Apply scroll and scissor
            // We use the scissor for scrolling
            // Because we use the scissor, our actual rect is now rect instead of parent_rect
            let off_x = 0.;
            // This calc decides whether scroll is in terms of pages or pixels
            let off_y = (scroll + start_pos - y_pos) / rect.h;
            let scale_x = 1. / rect.w;
            let scale_y = 1. / rect.h;
            let model = glam::Mat4::from_translation(glam::Vec3::new(off_x, off_y, 0.)) *
                glam::Mat4::from_scale(glam::Vec3::new(scale_x, scale_y, 1.));

            instrs.push(DrawInstruction::ApplyMatrix(model));

            instrs.push(DrawInstruction::Draw(mesh));
            //debug!(target: "ui::chatview", "mesh-{i}: {height} {current_height}");
        }

        let freed = std::mem::take(&mut msgbuf.freed);

        (instrs, freed)
    }

    pub async fn draw(&self, sg: &SceneGraph, parent_rect: &Rectangle) -> Option<DrawUpdate> {
        debug!(target: "ui::chatview", "ChatView::draw()");

        *self.parent_rect.lock().unwrap() = Some(parent_rect.clone());

        let rect = eval_rect(self.rect.clone(), parent_rect).expect("bad rect property");

        let mut msgbuf = self.msgbuf.lock().await;
        msgbuf.adjust_width(rect.w);

        let mut scroll = self.scroll.get();
        if let Some(scroll) = self.adjust_scroll(&mut msgbuf, scroll, rect.h).await {
            self.scroll.set(scroll);
        }

        let (mut mesh_instrs, freed) = self.get_meshes(&mut msgbuf, &rect).await;
        drop(msgbuf);

        let mut instrs = vec![DrawInstruction::ApplyViewport(rect)];
        instrs.append(&mut mesh_instrs);

        Some(DrawUpdate {
            key: self.dc_key,
            draw_calls: vec![(
                self.dc_key,
                DrawCall { instrs, dcs: vec![], z_index: self.z_index.get() },
            )],
            freed_textures: freed.textures,
            freed_buffers: freed.buffers,
        })
    }
}