use crate::models::data_structure::cpu::Cpu;

#[allow(dead_code)]
pub struct Host {
    pub name: String,
    pub cpus: Vec<Cpu>,
    pub network_address: String,
    pub resource_ids: Vec<u32>,
}