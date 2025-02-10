use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Deserialize, Serialize, PartialEq)]

pub enum State {
    Unknown,
    Waiting,
    Hold,
    ToLaunch,
    ToError,
    ToAckReservation,
    Launching,
    Running,
    Suspended,
    Resuming,
    Finishing,
    Terminated,
    Error,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Unknown => write!(f, "Unknown"),
            State::Waiting => write!(f, "Waiting"),
            State::Hold => write!(f, "Hold"),
            State::ToLaunch => write!(f, "ToLaunch"),
            State::ToError => write!(f, "ToError"),
            State::ToAckReservation => write!(f, "ToAckReservation"),
            State::Launching => write!(f, "Launching"),
            State::Running => write!(f, "Running"),
            State::Suspended => write!(f, "Suspended"),
            State::Resuming => write!(f, "Resuming"),
            State::Finishing => write!(f, "Finishing"),
            State::Terminated => write!(f, "Terminated"),
            State::Error => write!(f, "Error"),
        }
    }
}

#[derive(Clone)]

pub struct Job {
    pub id: u32,
    pub owner: String,
    pub state: State,
    pub command: String,
    pub walltime: i64,
    pub message: Option<String>,
    pub queue: String,
    pub assigned_resources: Vec<u32>,
    pub scheduled_start: i64,
    pub submission_time: i64,
    pub start_time: i64,
    pub stop_time: i64,
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
        println!("Start Time: {}", self.start_time);
        println!("Stop Time: {}", self.stop_time);
        println!("Exit Code: {:?}", self.exit_code);
    }
}