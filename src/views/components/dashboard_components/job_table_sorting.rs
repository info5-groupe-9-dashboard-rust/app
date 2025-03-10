use crate::models::data_structure::job::JobState;

/**
 * Enum for sorting keys
 */
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
    WallTime,
}

/**
 * Implementation of the sorting keys
 */
impl SortKey {
    /**
     * Sorts the jobs based on the selected key
     */
    pub fn sort_jobs<T>(self, jobs: &mut Vec<T>, ascending: bool)
    where
        T: JobSortable,
    {
        let comparator = |a: &T, b: &T| {
            let cmp = match self {
                SortKey::Id => a.get_id().cmp(&b.get_id()), // Compare the job id
                SortKey::Owner => a.get_owner().cmp(&b.get_owner()), // Compare the job owner
                SortKey::State => a.get_state().cmp(&b.get_state()), // Compare the job state
                SortKey::StartTime => a.get_start_time().cmp(&b.get_start_time()), // Compare the job start time
                SortKey::WallTime => a.get_walltime().cmp(&b.get_walltime()), // Compare the job walltime
                SortKey::Queue => a.get_queue().cmp(&b.get_queue()), // Compare the job queue
                SortKey::Command => a.get_command().cmp(&b.get_command()), // Compare the job command
                SortKey::Message => a.get_message().cmp(&b.get_message()), // Compare the job message
                SortKey::SubmissionTime => a.get_submission_time().cmp(&b.get_submission_time()), // Compare the job submission time
                SortKey::ScheduledStartTime => { // Compare the job scheduled start time
                    a.get_scheduled_start().cmp(&b.get_scheduled_start())
                }
                SortKey::StopTime => a.get_stop_time().cmp(&b.get_stop_time()), // Compare the job stop time
                SortKey::ExitCode => a.get_exit_code().cmp(&b.get_exit_code()), // Compare the job exit code
                SortKey::Clusters => a.get_clusters().cmp(&b.get_clusters()), // Compare the job clusters
            };

            // If the sorting is ascending, return the comparison, otherwise return the reverse comparison
            if ascending {
                cmp
            } else {
                cmp.reverse()
            }
        };
        
        // Sort the jobs based on the comparator
        jobs.sort_by(comparator);
    }
}

/**
 * Trait for sortable jobs
 */
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
    fn get_end_date(&self) -> i64;
}
