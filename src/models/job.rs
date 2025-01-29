use serde_json::Value;
use std::fs::File;
use std::io::Read;

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
    pub fn new(
        id: u32,
        owner: String,
        state: String,
        command: String,
        walltime: u64,
        message: Option<String>,
        queue: String,
        assigned_resources: Vec<u32>,
        scheduled_start: u64,
        submission_time: u64,
        exit_code: Option<i32>,
    ) -> Self {
        Job {
            id,
            owner,
            state,
            command,
            walltime,
            message,
            queue,
            assigned_resources,
            scheduled_start,
            submission_time,
            exit_code,
        }
    }

    pub fn from_json(json: &serde_json::Value) -> Self {
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

    pub fn display(&self) {
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

    pub fn from_json_value(json: &Value) -> Self {
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
    
    pub fn get_jobs_from_json(file_path: &str) -> Vec<Self> {
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
                jobs.push(Job::from_json_value(&value));
            }
        }
    
        jobs
    }
}