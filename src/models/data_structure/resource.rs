#[derive(Clone)]
pub enum ResourceState {
    Dead,
    Alive,
    Absent,
    Unknown,
}

#[derive(Clone)]
pub struct Resource {
    pub id: i32,
    pub state: ResourceState,
    pub thread_count: i32,
}