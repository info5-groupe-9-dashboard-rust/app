use super::{cluster::Cluster, job::JobState};

#[derive(Default, Debug)]

pub struct JobFilters {
    pub job_id_range: Option<(u32, u32)>,
    pub owners: Option<Vec<String>>,
    pub states: Option<Vec<JobState>>,
    pub scheduled_start_time: Option<i64>,
    pub wall_time: Option<i64>,
    pub clusters: Option<Vec<Cluster>>,
}

#[allow(dead_code)]
impl JobFilters {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn copy(filter: &JobFilters) -> Self {
        JobFilters {
            job_id_range: filter.job_id_range,
            owners: filter.owners.clone(),
            states: filter.states.clone(),
            scheduled_start_time: filter.scheduled_start_time,
            wall_time: filter.wall_time,
            clusters: filter.clusters.clone(),
        }
    }

    pub fn set_job_id_range(&mut self, start_id: u32, end_id: u32) {
        self.job_id_range = Some((start_id, end_id));
    }

    pub fn set_owners(&mut self, owners: Option<Vec<String>>) {
        self.owners = owners;
    }

    pub fn set_states(&mut self, states: Option<Vec<JobState>>) {
        self.states = states;
    }

    pub fn set_scheduled_start_time(&mut self, scheduled_start_time: i64) {
        self.scheduled_start_time = Some(scheduled_start_time);
    }

    pub fn set_wall_time(&mut self, wall_time: i64) {
        self.wall_time = Some(wall_time);
    }

    pub fn set_clusters(&mut self, selected_clusters: Option<Vec<Cluster>>) {
        self.clusters = selected_clusters;
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
