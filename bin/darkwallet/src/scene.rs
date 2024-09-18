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

use async_channel::Sender;
use async_lock::RwLock as AsyncRwLock;
use async_trait::async_trait;
use darkfi_serial::{FutAsyncWriteExt, SerialDecodable, SerialEncodable};
use std::{
    collections::VecDeque,
    fmt,
    future::Future,
    str::FromStr,
    sync::{Arc, RwLock as SyncRwLock, Weak},
};

use crate::{
    error::{Error, Result},
    prop::{Property, PropertyPtr, Role},
    ui,
};

pub struct ScenePath(VecDeque<String>);

impl<S: Into<String>> From<S> for ScenePath {
    fn from(path: S) -> Self {
        let path: String = path.into();
        (&path).parse().expect("invalid ScenePath &str")
    }
}

impl fmt::Display for ScenePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/")?;
        for token in &self.0 {
            write!(f, "{}/", token)?;
        }
        Ok(())
    }
}

impl FromStr for ScenePath {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.is_empty() || s.chars().nth(0).unwrap() != '/' {
            return Err(Error::InvalidScenePath);
        }
        if s == "/" {
            return Ok(ScenePath(VecDeque::new()));
        }

        let mut tokens = s.split('/');
        // Should start with a /
        let initial = tokens.next().expect("should not be empty");
        if !initial.is_empty() {
            return Err(Error::InvalidScenePath);
        }

        let mut path = VecDeque::new();
        for token in tokens {
            // There should not be any double slashes //
            if token.is_empty() {
                return Err(Error::InvalidScenePath);
            }
            path.push_back(token.to_string());
        }
        Ok(ScenePath(path))
    }
}

pub type SceneNodePtr = Arc<SceneNode>;
pub type SceneNodeWeak = Weak<SceneNode>;

pub type SceneNodeId = u32;

#[derive(Debug, Copy, Clone, PartialEq, SerialEncodable, SerialDecodable)]
#[repr(u8)]
pub enum SceneNodeType {
    Null = 0,
    Root = 1,
    Window = 2,
    WindowInput = 6,
    Keyboard = 7,
    Mouse = 8,
    Layer = 3,
    Object = 4,
    VectorArt = 5,
    Text = 9,
    Texture = 13,
    Fonts = 10,
    Font = 11,
    Plugins = 14,
    Plugin = 15,
    ChatView = 16,
    EditBox = 17,
    Image = 18,
    Button = 19,
}

pub struct SceneNode {
    pub name: String,
    pub id: SceneNodeId,
    pub typ: SceneNodeType,
    parent: SyncRwLock<Option<Weak<Self>>>,
    children: SyncRwLock<Vec<SceneNodePtr>>,
    pub props: Vec<PropertyPtr>,
    pub sigs: Vec<Signal>,
    pub methods: Vec<Method>,
    pub pimpl: Pimpl,
}

impl SceneNode {
    pub fn root() -> SceneNodePtr {
        Arc::new(Self::new("", SceneNodeType::Root))
    }

    pub fn new<S: Into<String>>(name: S, typ: SceneNodeType) -> Self {
        Self {
            name: name.into(),
            id: rand::random(),
            typ,
            parent: SyncRwLock::new(None),
            children: SyncRwLock::new(vec![]),
            props: vec![],
            sigs: vec![],
            methods: vec![],
            pimpl: Pimpl::Null,
        }
    }

    pub async fn setup<F, Fut>(self, pimpl_fn: F) -> SceneNodePtr
    where
        F: FnOnce(SceneNodeWeak) -> Fut,
        Fut: Future<Output = Pimpl>,
    {
        let mut self_ = Arc::new(self);
        let weak_self = Arc::downgrade(&self_);
        let pimpl = pimpl_fn(weak_self).await;
        // Arc::new_cyclic() doesnt allow async so we do this instead
        unsafe {
            Arc::get_mut_unchecked(&mut self_).pimpl = pimpl;
        }
        self_
    }

    pub fn link(&self, child: SceneNodePtr) {
        let mut childs_parent = child.parent.write().unwrap();
        assert!(childs_parent.is_none());
        *childs_parent = Some(Arc::downgrade(&child));
        drop(childs_parent);

        let mut children = self.children.write().unwrap();
        children.push(child);
    }

    pub fn get_children(&self) -> Vec<SceneNodePtr> {
        self.children.read().unwrap().clone()
    }

    pub fn lookup_node<P: Into<ScenePath>>(self: Arc<Self>, path: P) -> Option<SceneNodePtr> {
        let path: ScenePath = path.into();
        let mut path = path.0;
        if path.is_empty() {
            return Some(self)
        }
        let child_name = path.pop_front().unwrap();
        for child in self.get_children() {
            if child.name == child_name {
                let path = ScenePath(path);
                return child.lookup_node(path)
            }
        }
        None
    }

    fn has_property(&self, name: &str) -> bool {
        self.props.iter().any(|prop| prop.name == name)
    }
    pub fn add_property(&mut self, prop: Property) -> Result<()> {
        if self.has_property(&prop.name) {
            return Err(Error::PropertyAlreadyExists);
        }
        self.props.push(Arc::new(prop));
        Ok(())
    }

    pub fn get_property(&self, name: &str) -> Option<PropertyPtr> {
        self.props.iter().find(|prop| prop.name == name).map(|prop| prop.clone())
    }

    // Convenience methods
    pub fn get_property_bool(&self, name: &str) -> Result<bool> {
        self.get_property(name).ok_or(Error::PropertyNotFound)?.get_bool(0)
    }
    pub fn get_property_u32(&self, name: &str) -> Result<u32> {
        self.get_property(name).ok_or(Error::PropertyNotFound)?.get_u32(0)
    }
    pub fn get_property_f32(&self, name: &str) -> Result<f32> {
        self.get_property(name).ok_or(Error::PropertyNotFound)?.get_f32(0)
    }
    pub fn get_property_str(&self, name: &str) -> Result<String> {
        self.get_property(name).ok_or(Error::PropertyNotFound)?.get_str(0)
    }
    pub fn get_property_enum(&self, name: &str) -> Result<String> {
        self.get_property(name).ok_or(Error::PropertyNotFound)?.get_enum(0)
    }
    pub fn get_property_node_id(&self, name: &str) -> Result<SceneNodeId> {
        self.get_property(name).ok_or(Error::PropertyNotFound)?.get_node_id(0)
    }
    // Setters
    pub fn set_property_bool(&self, role: Role, name: &str, val: bool) -> Result<()> {
        self.get_property(name).ok_or(Error::PropertyNotFound)?.set_bool(role, 0, val)
    }
    pub fn set_property_u32(&self, role: Role, name: &str, val: u32) -> Result<()> {
        self.get_property(name).ok_or(Error::PropertyNotFound)?.set_u32(role, 0, val)
    }
    pub fn set_property_f32(&self, role: Role, name: &str, val: f32) -> Result<()> {
        self.get_property(name).ok_or(Error::PropertyNotFound)?.set_f32(role, 0, val)
    }
    pub fn set_property_str<S: Into<String>>(&self, role: Role, name: &str, val: S) -> Result<()> {
        self.get_property(name).ok_or(Error::PropertyNotFound)?.set_str(role, 0, val)
    }
    pub fn set_property_node_id(&self, role: Role, name: &str, val: SceneNodeId) -> Result<()> {
        self.get_property(name).ok_or(Error::PropertyNotFound)?.set_node_id(role, 0, val)
    }

    pub fn add_signal<S: Into<String>>(
        &mut self,
        name: S,
        desc: S,
        fmt: Vec<(S, S, CallArgType)>,
    ) -> Result<()> {
        let name = name.into();
        if self.has_signal(&name) {
            return Err(Error::SignalAlreadyExists);
        }
        let fmt = fmt
            .into_iter()
            .map(|(n, d, t)| CallArg { name: n.into(), desc: d.into(), typ: t })
            .collect();
        self.sigs.push(Signal {
            name: name.into(),
            desc: desc.into(),
            fmt,
            slots: vec![],
            freed: vec![],
        });
        Ok(())
    }

    fn has_signal(&self, name: &str) -> bool {
        self.sigs.iter().any(|sig| sig.name == name)
    }

    pub fn add_method<S: Into<String>>(
        &mut self,
        name: S,
        args: Vec<(S, S, CallArgType)>,
        result: Vec<(S, S, CallArgType)>,
        method_fn: MethodRequestFn,
    ) -> Result<()> {
        let name = name.into();
        if self.has_method(&name) {
            return Err(Error::MethodAlreadyExists);
        }
        let args = args
            .into_iter()
            .map(|(n, d, t)| CallArg { name: n.into(), desc: d.into(), typ: t })
            .collect();
        let result = result
            .into_iter()
            .map(|(n, d, t)| CallArg { name: n.into(), desc: d.into(), typ: t })
            .collect();
        self.methods.push(Method { name: name.into(), args, result, method_fn });
        Ok(())
    }

    fn has_method(&self, name: &str) -> bool {
        self.methods.iter().any(|sig| sig.name == name)
    }
}

impl std::fmt::Debug for SceneNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}':{}", self.name, self.id)
    }
}

#[derive(Debug, Clone, SerialEncodable, SerialDecodable)]
pub enum CallArgType {
    Uint32,
    Uint64,
    Bool,
    Str,
    Hash,
}

#[derive(Debug, Clone, SerialEncodable, SerialDecodable)]
pub struct CallArg {
    pub name: String,
    pub desc: String,
    pub typ: CallArgType,
}

pub type SlotId = u32;

pub struct Slot {
    pub name: String,
    pub notify: Sender<Vec<u8>>,
}

pub struct Signal {
    pub name: String,
    #[allow(dead_code)]
    pub desc: String,
    #[allow(dead_code)]
    pub fmt: Vec<CallArg>,
    slots: Vec<Slot>,
    freed: Vec<SlotId>,
}

type MethodRequestFn = Box<dyn Fn(Vec<u8>, MethodResponseFn) + Send + Sync>;
pub type MethodResponseFn = Box<dyn Fn(Result<Vec<u8>>) + Send + Sync>;

pub struct Method {
    pub name: String,
    pub args: Vec<CallArg>,
    pub result: Vec<CallArg>,
    method_fn: MethodRequestFn,
}

pub enum Pimpl {
    Null,
    Window(ui::WindowPtr),
    Layer(ui::LayerPtr),
    VectorArt(ui::VectorArtPtr),
    Text(ui::TextPtr),
    //EditBox(ui::EditBoxPtr),
    //ChatView(ui::ChatViewPtr),
    Image(ui::ImagePtr),
    //Button(ui::ButtonPtr),
}
