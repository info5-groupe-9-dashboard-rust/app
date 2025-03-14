use crate::models::data_structure::job::{Job, JobState};
use crate::models::data_structure::resource::ResourceState;
use crate::models::data_structure::strata::Strata;
use crate::models::utils::utils::convert_id_to_color;
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::process::Command;

#[cfg(not(target_arch = "wasm32"))]
use chrono::{DateTime, Local};

/**
 * Test SSH connection to the specified host
 */
pub fn test_connection(host: &str) -> Result<(), String> {
    let ssh_test = Command::new("ssh")
        .args([host, "true"])
        .status();

    match ssh_test {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(format!("SSH command failed with status: {}", status)),
        Err(e) => Err(format!("Connection test failed: {}", e)),
    }
}

/**
 * Get the jobs for the specified period
 * Command: oarstat -J -g "YYYY-MM-DD hh:mm:ss, YYYY-MM-DD hh:mm:ss" > /tmp/data.json
 * @param start_date: Start date of the period
 * @param end_date: End date of the period
 * @return List of jobs
 */
#[cfg(not(target_arch = "wasm32"))]
pub fn get_current_jobs_for_period(start_date: DateTime<Local>, end_date: DateTime<Local>) -> bool {
    // Add a margin to the interval
    let interval = end_date - start_date;
    let margin = interval.num_seconds() * 30 / 100;
    let start_date = start_date - chrono::Duration::seconds(margin);
    let end_date = end_date + chrono::Duration::seconds(margin);

    // Test connection first
    if test_connection("grenoble.g5k") != Ok(()) {
        return false;
    }

    // Check if Data folder exists
    let data_folder = std::path::Path::new("./data");
    if !data_folder.exists() {
        std::fs::create_dir(data_folder).expect("Unable to create data folder");
    }

    // Execute SSH command to generate JSON file and redirect output
    let ssh_status = Command::new("ssh")
        .args([
            "grenoble.g5k",
            &format!(
                "oarstat -J -g \"{}, {}\"",
                start_date.format("%Y-%m-%d %H:%M:%S"),
                end_date.format("%Y-%m-%d %H:%M:%S")
            ),
        ])
        .output()
        .and_then(|output| std::fs::write("./data/data.json", output.stdout));

    if let Err(e) = ssh_status {
        println!("Failed to execute SSH command: {}", e);
        return false;
    }

    true
}

pub fn get_jobs_from_json(file_path: &str) -> Vec<Job> {
    let file_res = File::open(file_path);

    let mut file = match file_res {
        Ok(file) => file,
        Err(error) => {
            println!("Unable to open file: {}", error);
            return Vec::new();
        }
    };

    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Unable to read string");

    let json: Value = serde_json::from_str(&data).expect("Unable to parse JSON");
    let mut jobs = Vec::new();

    if let Some(jobs_section) = json.get("jobs") {
        if let Value::Object(map) = jobs_section {
            for (_, value) in map {
                jobs.push(from_json_value(&value));
            }
        }
    }

    jobs
}

pub fn get_resources_from_json(file_path: &str) -> Vec<Strata> {
    // Open the file
    let file_res = File::open(file_path);

    let mut file = match file_res {
        Ok(file) => file,
        Err(error) => {
            println!("Impossible d'ouvrir le fichier: {}", error);
            return Vec::new();
        }
    };

    // Read the file content
    let mut data = String::new();
    if let Err(e) = file.read_to_string(&mut data) {
        println!("Impossible de lire le fichier: {}", e);
        return Vec::new();
    }

    // Parse the JSON content
    let json: Value = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(e) => {
            println!("Impossible de parser le JSON: {}", e);
            return Vec::new();
        }
    };

    let mut resources = Vec::new();

    // Get the resources array
    if let Some(resources_array) = json.get("resources").and_then(|v| v.as_array()) {
        for resource_value in resources_array {
            // Try to parse the resource
            if let Ok(resource) = serde_json::from_value::<Strata>(resource_value.clone()) {
                resources.push(resource);
            }
        }
    }

    resources
}

pub fn parse_state_from_json(json_str: &str) -> Result<JobState, serde_json::Error> {
    serde_json::from_str(json_str)
}

fn from_json_value(json: &Value) -> Job {
    Job {
        id: json["id"]
            .as_str()
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0),
        owner: json["owner"].as_str().unwrap_or("unknown").to_string(),
        state: parse_state_from_json(&format!(
            "\"{}\"",
            json["state"].as_str().unwrap_or("unknown")
        ))
        .unwrap_or(JobState::Unknown),
        command: json["command"].as_str().unwrap_or("").to_string(),
        walltime: json["walltime"].as_i64().unwrap_or(0) as i64,
        message: json["message"].as_str().map(|s| s.to_string()),
        queue: json["queue"].as_str().unwrap_or("default").to_string(),
        assigned_resources: json["resource_id"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .filter_map(|v| v.as_str().and_then(|s| s.parse::<u32>().ok()))
            .collect(),
        scheduled_start: json["start_time"].as_i64().unwrap_or(0),
        start_time: json["start_time"].as_i64().unwrap_or(0),
        stop_time: json["stop_time"].as_i64().unwrap_or(0),
        submission_time: json["submission_time"].as_i64().unwrap_or(0),
        exit_code: json["exit_code"].as_i64().map(|n| n as i32),
        gantt_color: convert_id_to_color(
            json["id"]
                .as_str()
                .unwrap_or("0")
                .parse::<u32>()
                .unwrap_or(0),
        ),
        clusters: Vec::new(),
        hosts: Vec::new(),
        main_resource_state: ResourceState::Unknown,
    }
}
