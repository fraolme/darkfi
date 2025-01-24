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

use structopt::StructOpt;
use url::Url;

type BlacklistEntry = (String, Vec<String>, Vec<u16>);

/// Ban policies definitions.
///
/// If the ban policy is set to `Relaxed` will not ban peers in case
/// they send a message without a corresponding MessageDispatcher.
/// This is useful for nodes that may not be subscribed to protocols,
/// such as Lilith. For most uses this should be set to `Strict`.
///
/// TODO: this will be deprecated when we introduce the p2p resource
/// mananger.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BanPolicy {
    #[default]
    Strict,

    Relaxed,
}

/// P2P network settings. The scope of this is a P2P network instance
/// configured by the library user.
#[derive(Debug, Clone)]
pub struct Settings {
    /// Only used for debugging, compromises privacy when set
    pub node_id: String,
    /// P2P accept addresses the instance listens on for inbound connections
    pub inbound_addrs: Vec<Url>,
    /// P2P external addresses the instance advertises so other peers can
    /// reach us and connect to us, as long as inbound addrs are configured
    pub external_addrs: Vec<Url>,
    /// Peer nodes to manually connect to
    pub peers: Vec<Url>,
    /// Seed nodes to connect to for peer discovery and/or adversising our
    /// own external addresses
    pub seeds: Vec<Url>,
    /// Application version, used for convenient protocol matching
    pub app_version: semver::Version,
    /// Whitelisted network transports for outbound connections
    pub allowed_transports: Vec<String>,
    /// Allow transport mixing (e.g. Tor would be allowed to connect to `tcp://`)
    pub transport_mixing: bool,
    /// Outbound connection slots number, this many connections will be
    /// attempted. (This does not include manual connections)
    pub outbound_connections: usize,
    /// Inbound connection slots number, this many active listening connections
    /// will be allowed. (This does not include manual connections)
    pub inbound_connections: usize,
    /// Outbound connection timeout (in seconds)
    pub outbound_connect_timeout: u64,
    /// Exchange versions (handshake) timeout (in seconds)
    pub channel_handshake_timeout: u64,
    /// Ping-pong exchange execution interval (in seconds)
    pub channel_heartbeat_interval: u64,
    /// Allow localnet hosts
    pub localnet: bool,
    /// Cooling off time for peer discovery when unsuccessful
    pub outbound_peer_discovery_cooloff_time: u64,
    /// Time between peer discovery attempts
    pub outbound_peer_discovery_attempt_time: u64,
    /// P2P datastore path
    pub p2p_datastore: Option<String>,
    /// Hostlist storage path
    pub hostlist: Option<String>,
    /// Pause interval within greylist refinery process
    pub greylist_refinery_interval: u64,
    /// Percent of connections to come from the whitelist
    pub white_connect_percent: usize,
    /// Number of goldlist connections
    pub gold_connect_count: usize,
    /// If this is true, strictly follow the gold_connect_count and
    /// white_connect_percent settings. Otherwise, connect to greylist
    /// entries if we have no white or gold connections.
    pub slot_preference_strict: bool,
    /// Number of seconds with no connections after which refinery
    /// process is paused.
    pub time_with_no_connections: u64,
    /// Nodes to avoid interacting with for the duration of the program,
    /// in the format ["host", ["scheme", "scheme"], [port, port]]
    /// If scheme is left empty it will default to "tcp+tls".
    /// If ports are left empty all ports from this peer will be blocked.
    pub blacklist: Vec<BlacklistEntry>,
    /// Do not ban nodes that send messages without dispatchers if set
    /// to `Relaxed`. For most uses, should be set to `Strict`.
    pub ban_policy: BanPolicy,
}

impl Default for Settings {
    fn default() -> Self {
        let version = option_env!("CARGO_PKG_VERSION").unwrap_or("0.0.0");
        let app_version = semver::Version::parse(version).unwrap();

        Self {
            node_id: String::new(),
            inbound_addrs: vec![],
            external_addrs: vec![],
            peers: vec![],
            seeds: vec![],
            app_version,
            allowed_transports: vec!["tcp+tls".to_string()],
            transport_mixing: true,
            outbound_connections: 8,
            inbound_connections: 8,
            outbound_connect_timeout: 15,
            channel_handshake_timeout: 10,
            channel_heartbeat_interval: 30,
            localnet: false,
            outbound_peer_discovery_cooloff_time: 30,
            outbound_peer_discovery_attempt_time: 5,
            p2p_datastore: None,
            hostlist: None,
            greylist_refinery_interval: 15,
            white_connect_percent: 70,
            gold_connect_count: 2,
            slot_preference_strict: false,
            time_with_no_connections: 30,
            blacklist: vec![],
            ban_policy: BanPolicy::Strict,
        }
    }
}

// The following is used so we can have P2P settings configurable
// from TOML files.

/// Defines the network settings.
#[derive(Clone, Debug, serde::Deserialize, structopt::StructOpt, structopt_toml::StructOptToml)]
#[structopt()]
pub struct SettingsOpt {
    /// P2P accept address node listens to for inbound connections
    #[serde(default)]
    #[structopt(long = "accept")]
    pub inbound: Vec<Url>,

    /// Outbound connection slots number
    #[structopt(long = "outbound-slots")]
    pub outbound_connections: Option<usize>,

    /// Inbound connection slots number
    #[structopt(long = "inbound-slots")]
    pub inbound_connections: Option<usize>,

    /// P2P external addresses node advertises so other peers can
    /// reach us and connect to us, as long as inbound addresses
    /// are also configured
    #[serde(default)]
    #[structopt(long)]
    pub external_addrs: Vec<Url>,

    /// Peer nodes to manually connect to
    #[serde(default)]
    #[structopt(long)]
    pub peers: Vec<Url>,

    /// Seed nodes to connect to for peers retrieval and/or
    /// advertising our own external addresses
    #[serde(default)]
    #[structopt(long)]
    pub seeds: Vec<Url>,

    /// Connection establishment timeout in seconds
    #[structopt(skip)]
    pub outbound_connect_timeout: Option<u64>,

    /// Exchange versions (handshake) timeout in seconds
    #[structopt(skip)]
    pub channel_handshake_timeout: Option<u64>,

    /// Ping-pong exchange execution interval in seconds
    #[structopt(skip)]
    pub channel_heartbeat_interval: Option<u64>,

    /// Only used for debugging. Compromises privacy when set.
    #[serde(default)]
    #[structopt(skip)]
    pub node_id: String,

    /// Preferred transports for outbound connections
    #[serde(default)]
    #[structopt(long = "transports")]
    pub allowed_transports: Option<Vec<String>>,

    /// Allow transport mixing (e.g. Tor would be allowed to connect to `tcp://`)
    #[structopt(long)]
    pub transport_mixing: Option<bool>,

    /// If this is true, strictly follow the gold_connect_count and
    /// white_connect_percent settings. Otherwise, connect to greylist
    /// entries if we have no white or gold connections.
    #[serde(default)]
    #[structopt(long)]
    pub localnet: bool,

    /// Cooling off time for peer discovery when unsuccessful
    #[structopt(skip)]
    pub outbound_peer_discovery_cooloff_time: Option<u64>,

    /// Time between peer discovery attempts
    #[structopt(skip)]
    pub outbound_peer_discovery_attempt_time: Option<u64>,

    /// P2P datastore path
    #[serde(default)]
    #[structopt(long)]
    pub p2p_datastore: Option<String>,

    /// Hosts .tsv file to use
    #[serde(default)]
    #[structopt(long)]
    pub hostlist: Option<String>,

    /// Pause interval within greylist refinery process
    #[structopt(skip)]
    pub greylist_refinery_interval: Option<u64>,

    /// Number of whitelist connections
    #[structopt(skip)]
    pub white_connect_percent: Option<usize>,

    /// Number of goldlist connections
    #[structopt(skip)]
    pub gold_connect_count: Option<usize>,

    /// Allow localnet hosts
    #[serde(default)]
    #[structopt(long)]
    pub slot_preference_strict: bool,

    /// Number of seconds with no connections after which refinery
    /// process is paused.
    #[structopt(skip)]
    pub time_with_no_connections: Option<u64>,

    /// Nodes to avoid interacting with for the duration of the program,
    /// in the format ["host", ["scheme", "scheme"], [port, port]]
    /// If scheme is left empty it will default to "tcp+tls".
    /// If ports are left empty all ports from this peer will be blocked.
    #[serde(default)]
    #[structopt(skip)]
    pub blacklist: Vec<BlacklistEntry>,

    /// Do not ban nodes that send messages without dispatchers if set
    /// to `Relaxed`. For most uses, should be set to `Strict`.
    #[serde(default)]
    #[structopt(skip)]
    pub ban_policy: BanPolicy,
}

impl From<SettingsOpt> for Settings {
    fn from(opt: SettingsOpt) -> Self {
        let def = Settings::default();

        Self {
            node_id: opt.node_id,
            inbound_addrs: opt.inbound,
            external_addrs: opt.external_addrs,
            peers: opt.peers,
            seeds: opt.seeds,
            app_version: def.app_version,
            allowed_transports: opt.allowed_transports.unwrap_or(def.allowed_transports),
            transport_mixing: opt.transport_mixing.unwrap_or(def.transport_mixing),
            outbound_connections: opt.outbound_connections.unwrap_or(def.outbound_connections),
            inbound_connections: opt.inbound_connections.unwrap_or(def.inbound_connections),
            outbound_connect_timeout: opt
                .outbound_connect_timeout
                .unwrap_or(def.outbound_connect_timeout),
            channel_handshake_timeout: opt
                .channel_handshake_timeout
                .unwrap_or(def.channel_handshake_timeout),
            channel_heartbeat_interval: opt
                .channel_heartbeat_interval
                .unwrap_or(def.channel_heartbeat_interval),
            localnet: opt.localnet,
            outbound_peer_discovery_cooloff_time: opt
                .outbound_peer_discovery_cooloff_time
                .unwrap_or(def.outbound_peer_discovery_cooloff_time),
            outbound_peer_discovery_attempt_time: opt
                .outbound_peer_discovery_attempt_time
                .unwrap_or(def.outbound_peer_discovery_attempt_time),
            p2p_datastore: opt.p2p_datastore,
            hostlist: opt.hostlist,
            greylist_refinery_interval: opt
                .greylist_refinery_interval
                .unwrap_or(def.greylist_refinery_interval),
            white_connect_percent: opt.white_connect_percent.unwrap_or(def.white_connect_percent),
            gold_connect_count: opt.gold_connect_count.unwrap_or(def.gold_connect_count),
            slot_preference_strict: opt.slot_preference_strict,
            time_with_no_connections: opt
                .time_with_no_connections
                .unwrap_or(def.time_with_no_connections),
            blacklist: opt.blacklist,
            ban_policy: opt.ban_policy,
        }
    }
}
