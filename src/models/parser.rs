
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::process::Command;
use super::job::Job;

/**
 * Get the current json file and parse it to get the jobs
 */
pub fn get_current_jobs() -> Vec<Job> {
    
    // Execute SSH command to generate JSON file
    let ssh_status = Command::new("ssh")
        .args(["grenoble.g5k", "oarstat -J > /tmp/data.json"])
        .status();

    if let Err(e) = ssh_status {
        println!("Failed to execute SSH command: {}", e);
        return Vec::new();
    }

    // Check if Data folder exists
    let data_folder = std::path::Path::new("./data");
    if !data_folder.exists() {
        std::fs::create_dir(data_folder).expect("Unable to create data folder");
    }

    // Execute SCP command to copy the file
    let scp_status = Command::new("scp")
        .args(["grenoble.g5k:/tmp/data.json", "./data/data.json"])
        .status();

    if let Err(e) = scp_status {
        println!("Failed to copy file via SCP: {}", e);
        return Vec::new();
    }

    // Read the jobs from the downloaded JSON file
    get_jobs_from_json("./data/data.json")
}


fn get_jobs_from_json(file_path: &str) -> Vec<Job> {
    let file_res = File::open(file_path);

    let mut file = match file_res {
        Ok(file) => file,
        Err(error) => {
            println!("Unable to open file: {}", error);
            return Vec::new();
        }
    };

    let mut data = String::new();
    file.read_to_string(&mut data).expect("Unable to read string");

    let json: Value = serde_json::from_str(&data).expect("Unable to parse JSON");
    let mut jobs = Vec::new();

    if let Value::Object(map) = json {
        for (_, value) in map {
            jobs.push(from_json_value(&value));
        }
    }

    jobs
}

fn from_json_value(json: &Value) -> Job {
    Job {
        id: json["id"].as_u64().unwrap() as u32,
        owner: json["owner"].as_str().unwrap().to_string(),
        state: json["state"].as_str().unwrap().to_string(),
        command: json["command"].as_str().unwrap_or("").to_string(),
        walltime: json["walltime"].as_u64().unwrap() as u64,
        message: json["message"].as_str().map(|s| s.to_string()),
        queue: json["queue"].as_str().unwrap().to_string(),
        assigned_resources: json["assigned_resources"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|v| v.as_u64().map(|n| n as u32))
            .collect(),
        scheduled_start: json["scheduled_start"].as_u64().unwrap(),
        submission_time: json["submission_time"].as_u64().unwrap(),
        exit_code: json["exit_code"].as_i64().map(|n| n as i32),
    }
}