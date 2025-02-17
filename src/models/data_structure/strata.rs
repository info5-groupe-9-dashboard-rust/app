use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Strata {
    #[serde(default)]
    pub state_num: Option<i32>, // OK
    #[serde(default)]
    pub thread_count: Option<i32>, // OK
    #[serde(default)]
    pub rconsole: Option<String>, // OK
    #[serde(default)]
    pub memnode: Option<i64>, // OK
    #[serde(default)]
    pub cluster: Option<String>, // OK
    #[serde(default)]
    pub desktop_computing: Option<String>, // OK
    #[serde(default)]
    pub memcore: Option<i32>, // OK
    #[serde(default)]
    pub production: Option<String>, // OK
    #[serde(default)]
    pub eth_rate: Option<i32>, // OK
    #[serde(default)]
    pub chassis: Option<String>, // OK
    #[serde(default)]
    pub memcpu: Option<i64>, // OK
    #[serde(default)] 
    pub cluster_priority: Option<i32>, // OK
    #[serde(default)]
    pub gpu_model: Option<String>, // OK
    #[serde(default)]
    pub gpu_compute_capability: Option<String>, // OK
    #[serde(default)]
    pub core_count: Option<i32>, // OK
    #[serde(default)]
    pub next_state: Option<String>, // OK
    #[serde(default)]
    pub cpufreq: Option<String>, // OK
    #[serde(default)]
    pub comment: Option<String>, // OK
    #[serde(default)]
    pub core: Option<i32>, // OK
    #[serde(default)]
    pub cpuset: Option<String>, // OK
    #[serde(default)]
    pub suspended_jobs: Option<String>, // OK
    #[serde(default)]
    pub state: Option<String>, // OK
    #[serde(default)]
    pub ip: Option<String>, // OK
    #[serde(default)]
    pub network_address: Option<String>, // OK
    #[serde(default)]
    pub resource_id: Option<u32>, // OK
    #[serde(default)]
    pub host: Option<String>, // OK
    #[serde(default)]
    pub nodemodel: Option<String>, // OK
}