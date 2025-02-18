use crate::models::data_structure::resource::Resource;

#[derive(Clone, Debug, PartialEq)]

pub struct Cpu {
    pub name: String,
    pub resources: Vec<Resource>,
    pub chassis: String,
    pub core_count: i32,
    pub cpufreq: f32,
    pub resource_ids: Vec<u32>,
}
