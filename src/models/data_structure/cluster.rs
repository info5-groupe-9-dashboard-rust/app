use crate::models::data_structure::host::Host;

#[derive(Clone)]
pub struct Cluster {
    pub name: String,
    pub hosts: Vec<Host>,
    pub resource_ids: Vec<u32>,
}