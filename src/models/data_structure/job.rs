use crate::models::data_structure::resource::ResourceState;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum_macros::EnumIter;
use super::cluster;

#[derive(Clone, Deserialize, Serialize, PartialEq, EnumIter, Debug, Eq, PartialOrd, Ord, Hash)]
pub enum JobState {
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

impl fmt::Display for JobState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JobState::Unknown => write!(f, "Unknown"),
            JobState::Waiting => write!(f, "Waiting"),
            JobState::Hold => write!(f, "Hold"),
            JobState::ToLaunch => write!(f, "ToLaunch"),
            JobState::ToError => write!(f, "ToError"),
            JobState::ToAckReservation => write!(f, "ToAckReservation"),
            JobState::Launching => write!(f, "Launching"),
            JobState::Running => write!(f, "Running"),
            JobState::Suspended => write!(f, "Suspended"),
            JobState::Resuming => write!(f, "Resuming"),
            JobState::Finishing => write!(f, "Finishing"),
            JobState::Terminated => write!(f, "Terminated"),
            JobState::Error => write!(f, "Error"),
        }
    }
}

impl JobState {
    pub fn get_label(&self) -> String {
        match self {
            JobState::Unknown => t!("app.job_state.unknown").to_string(),
            JobState::Waiting => t!("app.job_state.waiting").to_string(),
            JobState::Hold => t!("app.job_state.hold").to_string(),
            JobState::ToLaunch => t!("app.job_state.to_launch").to_string(),
            JobState::ToError => t!("app.job_state.to_error").to_string(),
            JobState::ToAckReservation => t!("app.job_state.to_ack_reservation").to_string(),
            JobState::Launching => t!("app.job_state.launching").to_string(),
            JobState::Running => t!("app.job_state.running").to_string(),
            JobState::Suspended => t!("app.job_state.suspended").to_string(),
            JobState::Resuming => t!("app.job_state.resuming").to_string(),
            JobState::Finishing => t!("app.job_state.finishing").to_string(),
            JobState::Terminated => t!("app.job_state.terminated").to_string(),
            JobState::Error => t!("app.job_state.error").to_string(),
        }
    }

    pub fn get_color(&self) -> (egui::Color32, egui::Color32) {
        match self {
            JobState::Unknown => (
                egui::Color32::from_rgb(200, 200, 200), // Neutral Gray
                egui::Color32::from_rgb(120, 120, 120), // Darker Gray
            ),
            JobState::Waiting => (
                egui::Color32::from_rgb(135, 206, 250), // SkyBlue
                egui::Color32::from_rgb(30, 144, 255),  // DodgerBlue
            ),
            JobState::Hold => (
                egui::Color32::from_rgb(255, 236, 179), // Light Amber
                egui::Color32::from_rgb(255, 193, 7),   // Amber
            ),
            JobState::ToLaunch => (
                egui::Color32::from_rgb(178, 235, 242), // LightCyanBlue
                egui::Color32::from_rgb(38, 198, 218),  // CyanBlue
            ),
            JobState::ToError => (
                egui::Color32::from_rgb(255, 204, 203), // LightPink
                egui::Color32::from_rgb(244, 67, 54),   // Red
            ),
            JobState::ToAckReservation => (
                egui::Color32::from_rgb(224, 191, 255), // Soft Purple
                egui::Color32::from_rgb(156, 39, 176),  // Purple
            ),
            JobState::Launching => (
                egui::Color32::from_rgb(197, 255, 198), // MintGreen
                egui::Color32::from_rgb(56, 142, 60),   // DarkGreen
            ),
            JobState::Running => (
                egui::Color32::from_rgb(165, 214, 167), // Soft Green
                egui::Color32::from_rgb(67, 160, 71),   // Medium Green
            ),
            JobState::Suspended => (
                egui::Color32::from_rgb(255, 221, 147), // Soft Orange
                egui::Color32::from_rgb(255, 87, 34),   // Deep Orange
            ),
            JobState::Resuming => (
                egui::Color32::from_rgb(129, 199, 132), // Soft Teal Green
                egui::Color32::from_rgb(0, 150, 136),   // Teal Green
            ),
            JobState::Finishing => (
                egui::Color32::from_rgb(144, 202, 249), // Soft Blue
                egui::Color32::from_rgb(33, 150, 243),  // Bright Blue
            ),
            JobState::Terminated => (
                egui::Color32::from_rgb(200, 230, 255), // Soft End Blue
                egui::Color32::from_rgb(13, 71, 161),   // Deep Blue
            ),
            JobState::Error => (
                egui::Color32::from_rgb(255, 138, 128), // Soft Red
                egui::Color32::from_rgb(183, 28, 28),   // Dark Red
            ),
        }
    }
}

use crate::views::components::dashboard_components::job_table_sorting::JobSortable;

#[derive(Clone, Debug)]

pub struct Job {
    pub id: u32,
    pub owner: String,
    pub state: JobState,
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
    pub gantt_color: egui::Color32,
    pub clusters: Vec<String>,
    pub hosts: Vec<String>,
    pub main_resource_state: ResourceState,
}

impl JobSortable for Job {
    fn get_id(&self) -> &u32 {
        &self.id
    }

    fn get_owner(&self) -> &str {
        &self.owner
    }

    fn get_state(&self) -> &JobState {
        &self.state
    }

    fn get_start_time(&self) -> u64 {
        self.start_time as u64
    }

    fn get_walltime(&self) -> u64 {
        self.walltime as u64
    }

    fn get_queue(&self) -> &str {
        &self.queue
    }

    fn get_command(&self) -> &str {
        &self.command
    }

    fn get_message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    fn get_submission_time(&self) -> u64 {
        self.submission_time as u64
    }

    fn get_scheduled_start(&self) -> u64 {
        self.scheduled_start as u64
    }

    fn get_stop_time(&self) -> u64 {
        self.stop_time as u64
    }

    fn get_exit_code(&self) -> &Option<i32> {
        &self.exit_code
    }

    fn get_clusters(&self) -> &Vec<String> {
        &self.clusters
    }

    fn get_end_date(&self) -> i64 {
        if self.stop_time > 0 {
            self.stop_time
        } else {
            self.start_time + self.walltime
        }
    }
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
        println!("Gant Color: {:?}", self.gantt_color);
        println!("Cluster: {:?}", self.clusters);
        println!("Host: {:?}", self.hosts);
    }

    // Based on gantt color return a tuple of two colors (the second one is darker)
    pub fn get_gantt_color(&self) -> (egui::Color32, egui::Color32) {
        let r = self.gantt_color.r() as f32;
        let g = self.gantt_color.g() as f32;
        let b = self.gantt_color.b() as f32;

        let darker = |c: f32| -> u8 {
            let value = (c * 0.8) as u8;
            if value > 0 {
                value
            } else {
                1
            }
        };

        (
            egui::Color32::from_rgb(r as u8, g as u8, b as u8),
            egui::Color32::from_rgb(darker(r), darker(g), darker(b)),
        )
    }

    pub fn update_majority_resource_state(&mut self, clusters: &Vec<cluster::Cluster>) {
        let mut dead_count = 0;
        let mut alive_count = 0;
        let mut absent_count = 0;

        for cluster in clusters {
            for host in &cluster.hosts {
                for cpu in &host.cpus {
                    for resource in &cpu.resources {
                        if self.assigned_resources.contains(&resource.id) {
                            match resource.state {
                                ResourceState::Dead => dead_count += 1,
                                ResourceState::Alive => alive_count += 1,
                                ResourceState::Absent => absent_count += 1,
                                _ => (),
                            }
                        }
                    }
                }
            }
        }

        self.main_resource_state = if dead_count >= alive_count && dead_count >= absent_count {
            ResourceState::Dead
        } else if absent_count >= dead_count && absent_count >= alive_count {
            ResourceState::Absent
        } else if alive_count > dead_count && alive_count > absent_count {
            ResourceState::Alive
        } else {
            ResourceState::Unknown
        };
    }
}