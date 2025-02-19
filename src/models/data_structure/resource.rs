use std::fmt::Display;
#[derive(Debug, PartialEq)]

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

#[derive(Clone, Debug, PartialEq)]
pub struct Resource {
    pub id: u32,
    pub state: ResourceState,
    pub thread_count: i32,
}

impl Display for ResourceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceState::Dead => write!(f, "Dead"),
            ResourceState::Alive => write!(f, "Alive"),
            ResourceState::Absent => write!(f, "Absent"),
            ResourceState::Unknown => write!(f, "Unknown"),
        }
    }
}
