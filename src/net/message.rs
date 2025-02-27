/* This file is part of DarkFi (https://dark.fi)
 *
 * Copyright (C) 2020-2025 Dyne.org foundation
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

use darkfi_serial::{
    async_trait, serialize_async, AsyncDecodable, AsyncEncodable, SerialDecodable, SerialEncodable,
};
use std::net::Ipv6Addr;
use url::{Host, Url};

/// Generic message template.
pub trait Message: 'static + Send + Sync + AsyncDecodable + AsyncEncodable {
    const NAME: &'static str;
}

/// Generic serialized message template.
pub struct SerializedMessage {
    pub command: String,
    pub payload: Vec<u8>,
}

impl SerializedMessage {
    pub async fn new<M: Message>(message: &M) -> Self {
        Self { command: M::NAME.to_string(), payload: serialize_async(message).await }
    }
}

#[macro_export]
macro_rules! impl_p2p_message {
    ($st:ty, $nm:expr) => {
        impl Message for $st {
            const NAME: &'static str = $nm;
        }
    };
}

/// Outbound keepalive message.
#[derive(Debug, Copy, Clone, SerialEncodable, SerialDecodable)]
pub struct PingMessage {
    pub nonce: u16,
}
impl_p2p_message!(PingMessage, "ping");

/// Inbound keepalive message.
#[derive(Debug, Copy, Clone, SerialEncodable, SerialDecodable)]
pub struct PongMessage {
    pub nonce: u16,
}
impl_p2p_message!(PongMessage, "pong");

/// Requests address of outbound connecction.
#[derive(Debug, Clone, SerialEncodable, SerialDecodable)]
pub struct GetAddrsMessage {
    /// Maximum number of addresses with preferred
    /// transports to receive. Response vector will
    /// also containg addresses without the preferred
    /// transports, so its size will be 2 * max.
    pub max: u32,
    /// Preferred addresses transports
    pub transports: Vec<String>,
}
impl_p2p_message!(GetAddrsMessage, "getaddr");

/// Sends address information to inbound connection.
#[derive(Debug, Clone, SerialEncodable, SerialDecodable)]
pub struct AddrsMessage {
    pub addrs: Vec<(Url, u64)>,
}

impl_p2p_message!(AddrsMessage, "addr");

/// Requests version information of outbound connection.
#[derive(Debug, Clone, SerialEncodable, SerialDecodable)]
pub struct VersionMessage {
    /// Only used for debugging. Compromises privacy when set.
    pub node_id: String,
    /// Identifies protocol version being used by the node
    pub version: semver::Version,
    /// UNIX timestamp of when the VersionMessage was created.
    pub timestamp: u64,
    /// Network address of the node receiving this message (before
    /// resolving).
    pub connect_recv_addr: Url,
    /// Network address of the node receiving this message (after
    /// resolving). Optional because only used by outbound connections.
    pub resolve_recv_addr: Option<Url>,
    /// External address of the sender node, if it exists (empty
    /// otherwise).
    pub ext_send_addr: Vec<Url>,
    /// List of features consisting of a tuple of (services, version)
    /// to be enabled for this connection
    pub features: Vec<(String, u32)>,
}
impl_p2p_message!(VersionMessage, "version");

impl VersionMessage {
    pub(in crate::net) fn get_ipv6_addr(&self) -> Option<Ipv6Addr> {
        let host = self.connect_recv_addr.host()?;
        // Check the reported address is Ipv6
        match host {
            Host::Ipv6(addr) => Some(addr),
            _ => None,
        }
    }
}

/// Sends version information to inbound connection.
/// Response to `VersionMessage`.
#[derive(Debug, Clone, SerialEncodable, SerialDecodable)]
pub struct VerackMessage {
    /// App version
    pub app_version: semver::Version,
}
impl_p2p_message!(VerackMessage, "verack");
