use crate::models::data_structure::cpu::Cpu;

pub struct Host {
    pub name: String,
    pub cpus: Vec<Cpu>,
}