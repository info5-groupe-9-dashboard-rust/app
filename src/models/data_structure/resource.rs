use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resource {
    #[serde(default)]
    pub state_num: Option<i32>,
    #[serde(default)]
    pub thread_count: Option<i32>,
    #[serde(default)]
    pub rconsole: Option<String>,
    #[serde(default)]
    pub memnode: Option<i64>,
    #[serde(default)]
    pub cluster: Option<String>,
    #[serde(default)]
    pub desktop_computing: Option<String>,
    #[serde(default)]
    pub memcore: Option<i32>,
    #[serde(default)]
    pub production: Option<String>,
    #[serde(default)]
    pub eth_rate: Option<i32>,
    #[serde(default)]
    pub chassis: Option<String>,
    #[serde(default)]
    pub memcpu: Option<i64>,
    #[serde(default)] 
    pub cluster_priority: Option<i32>,
    #[serde(default)]
    pub gpu_model: Option<String>,
    #[serde(default)]
    pub gpu_compute_capability: Option<String>,
    #[serde(default)]
    pub core_count: Option<i32>,
    #[serde(default)]
    pub next_state: Option<String>,
    #[serde(default)]
    pub cpufreq: Option<String>,
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default)]
    pub core: Option<i32>,
    #[serde(default)]
    pub cpuset: Option<String>,
    #[serde(default)]
    pub suspended_jobs: Option<String>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub ip: Option<String>,
    #[serde(default)]
    pub network_address: Option<String>,
    #[serde(default)]
    pub resource_id: Option<i32>,
    #[serde(default)]
    pub host: Option<String>,
}