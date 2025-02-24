use crate::models::data_structure::host::Host;

use super::resource::ResourceState;

#[derive(Clone, Debug, PartialEq)]
pub struct Cluster {
    pub name: String,
    pub hosts: Vec<Host>,
    pub resource_ids: Vec<u32>,
    pub state: ResourceState,
}
