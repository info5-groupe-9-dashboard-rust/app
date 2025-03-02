use crate::models::data_structure::cpu::Cpu;
use crate::models::data_structure::resource::ResourceState;

#[derive(Clone, Debug, PartialEq)]

pub struct Host {
    pub name: String,
    pub cpus: Vec<Cpu>,
    pub network_address: String,
    pub resource_ids: Vec<u32>,
    pub state: ResourceState,
}
