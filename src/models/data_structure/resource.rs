pub enum ResourceState {
    Dead,
    Alive,
    Absent,
    Unknown,
}

impl Clone for ResourceState {
    fn clone(&self) -> Self {
        match self {
            ResourceState::Dead => ResourceState::Dead,
            ResourceState::Alive => ResourceState::Alive,
            ResourceState::Absent => ResourceState::Absent,
            ResourceState::Unknown => ResourceState::Unknown,
        }
    }
}


#[allow(dead_code)]
#[derive(Clone)]
pub struct Resource {
    pub id: u32,
    pub state: ResourceState,
    pub thread_count: i32,
}