#[derive(Clone)]
pub struct Job {
    pub id: u32,
    pub owner: String,
    pub state: String,
    pub command: String,
    pub walltime: u64,
    pub message: Option<String>,
    pub queue: String,
    pub assigned_resources: Vec<u32>,
    pub scheduled_start: u64,
    pub submission_time: u64,
    pub exit_code: Option<i32>,
}


impl Job {
    pub fn _display(&self) {
        println!("Job ID: {}", self.id);
        println!("Owner: {}", self.owner);
        println!("State: {}", self.state);
        println!("Command: {}", self.command);
        println!("Walltime: {}", self.walltime);
        println!("Message: {:?}", self.message);
        println!("Queue: {}", self.queue);
        println!("Assigned Resources: {:?}", self.assigned_resources);
        println!("Scheduled Start: {:?}", self.scheduled_start);
        println!("Submission Time: {}", self.submission_time);
        println!("Exit Code: {:?}", self.exit_code);
    }
}