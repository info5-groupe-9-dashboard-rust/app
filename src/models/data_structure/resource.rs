pub enum ResourceState {
    Dead,
    Alive,
    Absent,
    Unknown,
}

pub struct Resource {
    pub id: i32,
    pub state: ResourceState,
    pub thread_count: i32,
}