use std::path::PathBuf;

pub use libc::pid_t;
use serde::{Deserialize, Serialize};

use interface::engine::SchedulingMode;

type IResult<T> = Result<T, interface::Error>;

/// Description for loading/upgrading a plugin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDescriptor {
    /// Name of the plugin. It must match one of the valid EngineType defined in plugin's module.rs.
    pub name: String,
    /// The path of the plugin.
    pub lib_path: PathBuf,
    /// The path of the configuration file of this plugin. Should be a toml file.
    pub config_path: Option<PathBuf>,
    /// The configuration string.
    pub config_string: Option<String>,
}

/// Request for upgrading plugins.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeRequest {
    /// plugins to upgrade
    pub plugins: Vec<PluginDescriptor>,
    /// type of the plugins to upgrade,
    /// module or addon
    pub ty: PluginType,
    /// whether to flush the shared queues
    pub flush: bool,
    /// whether to suspend all engines
    /// within the same service subscription
    pub detach_subscription: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginType {
    Module,
    Addon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddonRequest {
    /// Ttarget user process
    pub pid: pid_t,
    /// Target service subscription
    pub sid: u64,
    /// addon engine type to attach/detach
    pub addon_engine: String,
    /// replacement for data path tx edges
    pub tx_channels_replacements: Vec<(String, String, usize, usize)>,
    /// replacement for data path rx edges
    pub rx_channels_replacements: Vec<(String, String, usize, usize)>,
    /// Which scheduling group should the addon belongs when attaching an addon,
    /// the group is identified as a set engines
    pub group: Vec<String>,
    /// The path of the configuration file of this plugin. Should be a toml file.
    pub config_path: Option<PathBuf>,
    /// The configuration string.
    pub config_string: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Request {
    /// New service subscription, scheduling mode and service name
    NewClient(SchedulingMode, String),
    /// Send a request to a specified engine, identified by the EngineId
    EngineRequest(u64, Vec<u8>),
    /// List all service subscriptions
    ListSubscription,
    /// Attach an addon to a service subscription
    AttachAddon(SchedulingMode, AddonRequest),
    /// Detach an addon from a service subscription
    DetachAddon(AddonRequest),
    /// Upgrade modules or plugins
    Upgrade(UpgradeRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceSubscriptionInfo {
    pub pid: pid_t,
    pub sid: u64,
    pub service: String,
    pub engines: Vec<(u64, String)>,
    pub addons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseKind {
    /// path of the engine's domain socket
    NewClient(PathBuf),
    ListSubscription(Vec<ServiceSubscriptionInfo>),
    /// .0: the requested scheduling mode
    /// .1: name of the OneShotServer
    /// .2: data path work queue capacity in bytes
    ConnectEngine {
        mode: SchedulingMode,
        one_shot_name: String,
        wq_cap: usize,
        cq_cap: usize,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response(pub IResult<ResponseKind>);