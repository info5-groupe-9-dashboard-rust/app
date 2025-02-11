use super::job::State;

#[derive(Default)]
pub struct JobFilters {
    pub job_id_range: Option<(u32, u32)>,
    pub owners: Option<Vec<String>>,
    pub states: Option<Vec<State>>,
    pub scheduled_start_time: Option<i64>,
    pub wall_time: Option<i64>,
}

impl JobFilters {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_job_id_range(&mut self, start_id: u32, end_id: u32) {
        self.job_id_range = Some((start_id, end_id));
    }

    pub fn set_owners(&mut self, owners: Vec<String>) {
        self.owners = Some(owners);
    }

    pub fn set_states(&mut self, states: Vec<State>) {
        self.states = Some(states);
    }

    pub fn set_scheduled_start_time(&mut self, scheduled_start_time: i64) {
        self.scheduled_start_time = Some(scheduled_start_time);
    }

    pub fn set_wall_time(&mut self, wall_time: i64) {
        self.wall_time = Some(wall_time);
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
