use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use interface::engine::EngineType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Node {
    pub id: String,
    #[serde(alias = "type")]
    pub engine_type: EngineType,
}

use std::hash::{Hash, Hasher};
impl Hash for Node {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Edges {
    pub egress: Vec<Vec<String>>,
    pub ingress: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Control {
    pub prefix: PathBuf,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RdmaTransportConfig {
    pub prefix: PathBuf,
    pub engine_basename: String,
    pub datapath_wq_depth: usize,
    pub datapath_cq_depth: usize,
    pub command_max_interval_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub log_env: String,
    pub default_log_level: String,
    pub modules: Vec<String>,
    pub control: Control,
    #[serde(alias = "transport-rdma")]
    pub transport_rdma: Option<RdmaTransportConfig>,
    pub node: Vec<Node>,
    pub edges: Edges,
}

impl Config {
    pub fn from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }
}