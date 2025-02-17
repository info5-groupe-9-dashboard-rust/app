use crate::models::data_structure::resource::Resource;

pub struct Cpu {
    pub name: String,
    pub resources: Vec<Resource>,
}