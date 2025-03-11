#[cfg(test)]
mod tests {
    use crate::models::data_structure::job::JobState;
    use crate::models::utils::parser::{get_jobs_from_json, get_resources_from_json};
    use std::fs;
    use std::io::Write;

    // Write JSON to a file with specific format that matches the expected input
    fn write_test_json_file(content: &str, filename: &str) -> String {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join(filename);

        // Write content to file
        let mut file = fs::File::create(&file_path).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write to file");

        // Return the absolute path as a string
        file_path.to_str().unwrap().to_string()
    }

    #[test]
    /// Tests that an empty jobs object returns an empty vector
    fn test_empty_array() {
        let json_content = r#"{"jobs":{}}"#;
        let file_path = write_test_json_file(json_content, "empty_jobs.json");

        let jobs = get_jobs_from_json(&file_path);

        fs::remove_file(file_path).unwrap_or_default();
        assert_eq!(jobs.len(), 0);
    }

    #[test]
    /// Tests basic job parsing with minimal valid job data
    fn test_basic_job_parsing() {
        // Format the JSON in the structure expected by the parser
        let json_content = r#"{
            "jobs": {
                "job1": {
                    "id": "1234",
                    "owner": "test_user",
                    "state": "Running",
                    "walltime": 3600,
                    "resource_id": ["101"],
                    "start_time": 1672527600
                }
            }
        }"#;

        let file_path = write_test_json_file(json_content, "basic_job.json");

        let jobs = get_jobs_from_json(&file_path);

        fs::remove_file(file_path).unwrap_or_default();

        // Just verify that we got some job back
        assert!(jobs.len() > 0, "Should parse at least one job");

        // If successful, add more detailed assertions
        if jobs.len() > 0 {
            let job = &jobs[0];
            assert_eq!(job.id, 1234);
            assert_eq!(job.owner, "test_user");
            assert_eq!(job.state, JobState::Running);
        }
    }

    #[test]
    /// Tests resource parsing with minimal valid resource data
    fn test_basic_resource_parsing() {
        // Note: get_resources_from_json expects an array of resources,
        // not an object map like job parsing does
        let json_content = r#"{
            "resources": [
                {
                    "resource_id": 101,
                    "state": "Alive",
                    "cluster": "cluster1",
                    "host": "host1"
                }
            ]
        }"#;

        let file_path = write_test_json_file(json_content, "basic_resource.json");

        let resources = get_resources_from_json(&file_path);

        fs::remove_file(file_path).unwrap_or_default();

        // Just verify that we got some resource back
        assert!(resources.len() > 0, "Should parse at least one resource");
    }

    #[test]
    /// Debug test to understand the expected JSON format
    fn test_debug_parser_behavior() {
        // Format JSON according to the expected structure from parser.rs
        let json_content = r#"{
            "jobs": {
                "job999": {
                    "id": "999", 
                    "owner": "debug", 
                    "state": "Running", 
                    "walltime": 3600,
                    "resource_id": ["101", "102"],
                    "start_time": 1672527600
                }
            }
        }"#;

        let file_path = write_test_json_file(json_content, "debug.json");

        // Debug info
        println!("Debug test: File path = {}", file_path);
        println!(
            "Debug test: File exists = {}",
            fs::metadata(&file_path).is_ok()
        );

        // Try to read file back to verify content
        let content =
            fs::read_to_string(&file_path).unwrap_or_else(|e| format!("Error reading: {}", e));
        println!("Debug test: File content = {}", content);

        let jobs = get_jobs_from_json(&file_path);
        println!("Debug test: Parsed jobs count = {}", jobs.len());
        if jobs.len() > 0 {
            println!("Debug test: First job ID = {}", jobs[0].id);
        }

        fs::remove_file(file_path).unwrap_or_default();
    }

    // Additional tests can be added once the basic structure works
}
