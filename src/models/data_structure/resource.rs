#[derive(Clone, Debug, PartialEq)]

pub enum ResourceState {
    Dead,
    Alive,
    Absent,
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]

pub struct Resource {
    pub id: u32,
    pub state: ResourceState,
    pub thread_count: i32,
}
