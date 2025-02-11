use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
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

impl State {

    pub fn get_label(&self) -> String {
        match self {
            State::Unknown => t!("app.job_state.unknown").to_string(),
            State::Waiting => t!("app.job_state.waiting").to_string(),
            State::Hold => t!("app.job_state.hold").to_string(),
            State::ToLaunch => t!("app.job_state.to_launch").to_string(),
            State::ToError => t!("app.job_state.to_error").to_string(),
            State::ToAckReservation => t!("app.job_state.to_ack_reservation").to_string(),
            State::Launching => t!("app.job_state.launching").to_string(),
            State::Running => t!("app.job_state.running").to_string(),
            State::Suspended => t!("app.job_state.suspended").to_string(),
            State::Resuming => t!("app.job_state.resuming").to_string(),
            State::Finishing => t!("app.job_state.finishing").to_string(),
            State::Terminated => t!("app.job_state.terminated").to_string(),
            State::Error => t!("app.job_state.error").to_string(),
        }
    }

    pub fn get_color(&self) -> (egui::Color32, egui::Color32) {
        match self {
            State::Unknown => (
                egui::Color32::from_rgb(200, 200, 200), // Neutral Gray
                egui::Color32::from_rgb(120, 120, 120), // Darker Gray
            ),
            State::Waiting => (
                egui::Color32::from_rgb(135, 206, 250), // SkyBlue
                egui::Color32::from_rgb(30, 144, 255), // DodgerBlue
            ),
            State::Hold => (
                egui::Color32::from_rgb(255, 236, 179), // Light Amber
                egui::Color32::from_rgb(255, 193, 7), // Amber
            ),
            State::ToLaunch => (
                egui::Color32::from_rgb(178, 235, 242), // LightCyanBlue
                egui::Color32::from_rgb(38, 198, 218), // CyanBlue
            ),
            State::ToError => (
                egui::Color32::from_rgb(255, 204, 203), // LightPink
                egui::Color32::from_rgb(244, 67, 54), // Red
            ),
            State::ToAckReservation => (
                egui::Color32::from_rgb(224, 191, 255), // Soft Purple
                egui::Color32::from_rgb(156, 39, 176), // Purple
            ),
            State::Launching => (
                egui::Color32::from_rgb(197, 255, 198), // MintGreen
                egui::Color32::from_rgb(56, 142, 60), // DarkGreen
            ),
            State::Running => (
                egui::Color32::from_rgb(165, 214, 167), // Soft Green
                egui::Color32::from_rgb(67, 160, 71), // Medium Green
            ),
            State::Suspended => (
                egui::Color32::from_rgb(255, 221, 147), // Soft Orange
                egui::Color32::from_rgb(255, 87, 34), // Deep Orange
            ),
            State::Resuming => (
                egui::Color32::from_rgb(129, 199, 132), // Soft Teal Green
                egui::Color32::from_rgb(0, 150, 136), // Teal Green
            ),
            State::Finishing => (
                egui::Color32::from_rgb(144, 202, 249), // Soft Blue
                egui::Color32::from_rgb(33, 150, 243), // Bright Blue
            ),
            State::Terminated => (
                egui::Color32::from_rgb(200, 230, 255), // Soft End Blue
                egui::Color32::from_rgb(13, 71, 161), // Deep Blue
            ),
            State::Error => (
                egui::Color32::from_rgb(255, 138, 128), // Soft Red
                egui::Color32::from_rgb(183, 28, 28), // Dark Red
            ),
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