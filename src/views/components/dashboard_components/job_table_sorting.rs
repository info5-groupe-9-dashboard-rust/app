use crate::models::data_structure::job::JobState;

#[derive(PartialEq, Copy, Clone)]
pub(crate) enum SortKey {
    Id,
    Owner,
    Queue,
    Command,
    State,
    Message,
    SubmissionTime,
    ScheduledStartTime,
    StartTime,
    StopTime,
    ExitCode,
    Clusters,
    WallTime
}

impl SortKey {
    pub fn sort_jobs<T>(self, jobs: &mut Vec<T>, ascending: bool)
    where
        T: JobSortable,
    {
        let comparator = |a: &T, b: &T| {
            let cmp = match self {
                SortKey::Id => a.get_id().cmp(&b.get_id()),
                SortKey::Owner => a.get_owner().cmp(&b.get_owner()),
                SortKey::State => a.get_state().cmp(&b.get_state()),
                SortKey::StartTime => a.get_start_time().cmp(&b.get_start_time()),
                SortKey::WallTime => a.get_walltime().cmp(&b.get_walltime()),
                SortKey::Queue => a.get_queue().cmp(&b.get_queue()),
                SortKey::Command => a.get_command().cmp(&b.get_command()),
                SortKey::Message => a.get_message().cmp(&b.get_message()),
                SortKey::SubmissionTime => a.get_submission_time().cmp(&b.get_submission_time()),
                SortKey::ScheduledStartTime => a.get_scheduled_start().cmp(&b.get_scheduled_start()),
                SortKey::StopTime => a.get_stop_time().cmp(&b.get_stop_time()),
                SortKey::ExitCode => a.get_exit_code().cmp(&b.get_exit_code()),
                SortKey::Clusters => a.get_clusters().cmp(&b.get_clusters()),
            };
            if ascending { cmp } else { cmp.reverse() }
        };
        jobs.sort_by(comparator);
    }
}

pub trait JobSortable {
    fn get_id(&self) -> &u32;
    fn get_owner(&self) -> &str;
    fn get_state(&self) -> &JobState;
    fn get_start_time(&self) -> u64;
    fn get_walltime(&self) -> u64;
    fn get_queue(&self) -> &str;
    fn get_command(&self) -> &str;
    fn get_message(&self) -> Option<&str>;
    fn get_submission_time(&self) -> u64;
    fn get_scheduled_start(&self) -> u64;
    fn get_stop_time(&self) -> u64;
    fn get_exit_code(&self) -> &Option<i32>;
    fn get_clusters(&self) -> &Vec<String>;
}