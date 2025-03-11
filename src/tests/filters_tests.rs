#[cfg(test)]
mod tests {
    use crate::models::data_structure::application_context::ApplicationContext;
    use crate::models::data_structure::cluster::Cluster;
    use crate::models::data_structure::cpu::Cpu;
    use crate::models::data_structure::filters::JobFilters;
    use crate::models::data_structure::host::Host;
    use crate::models::data_structure::job::{Job, JobState};
    use crate::models::data_structure::resource::{Resource, ResourceState};
    use egui::Color32;

    // Helper function to create a set of test jobs with different properties
    fn create_test_jobs() -> Vec<Job> {
        vec![
            // Job 1: owner="user1", state=Running, resources=[101, 102]
            Job {
                id: 1001,
                owner: "user1".to_string(),
                state: JobState::Running,
                walltime: 3600,
                start_time: 1672527600, // 2023-01-01 00:00:00
                scheduled_start: 1672527600,
                assigned_resources: vec![101, 102],
                main_resource_state: ResourceState::Alive,
                clusters: vec!["cluster1".to_string()], // Pre-populate for test simplicity
                hosts: vec!["host1".to_string(), "host2".to_string()], // Pre-populate for test simplicity
                command: "test_command1".to_string(),
                message: Some("test_message1".to_string()),
                queue: "default".to_string(),
                submission_time: 1672527500,
                stop_time: 0,
                exit_code: Some(0),
                gantt_color: Color32::from_rgb(10, 100, 100),
            },
            // Job 2: owner="user2", state=Waiting, resources=[103]
            Job {
                id: 1002,
                owner: "user2".to_string(),
                state: JobState::Waiting,
                walltime: 7200,
                start_time: 1672614000, // 2023-01-02 00:00:00
                scheduled_start: 1672614000,
                assigned_resources: vec![103],
                main_resource_state: ResourceState::Alive,
                clusters: vec!["cluster2".to_string()], // Pre-populate for test simplicity
                hosts: vec!["host3".to_string()],       // Pre-populate for test simplicity
                command: "test_command2".to_string(),
                message: Some("test_message2".to_string()),
                queue: "high_priority".to_string(),
                submission_time: 1672613900,
                stop_time: 0,
                exit_code: None,
                gantt_color: Color32::from_rgb(10, 100, 100),
            },
            // Job 3: owner="user1", state=Terminated, resources=[104]
            Job {
                id: 1003,
                owner: "user1".to_string(),
                state: JobState::Terminated,
                walltime: 1800,
                start_time: 1672700400, // 2023-01-03 00:00:00
                scheduled_start: 1672700400,
                assigned_resources: vec![104],
                main_resource_state: ResourceState::Alive,
                clusters: vec!["cluster2".to_string()], // Pre-populate for test simplicity
                hosts: vec!["host4".to_string()],       // Pre-populate for test simplicity
                command: "test_command3".to_string(),
                message: Some("test_message3".to_string()),
                queue: "default".to_string(),
                submission_time: 1672700300,
                stop_time: 1672702200, // 30 minutes after start
                exit_code: Some(0),
                gantt_color: Color32::from_rgb(10, 100, 100),
            },
        ]
    }

    // Helper function to create test clusters
    fn create_test_clusters() -> Vec<Cluster> {
        vec![
            // Cluster 1 with resources 101, 102
            Cluster {
                name: "cluster1".to_string(),
                hosts: vec![
                    // Host 1 with resource 101
                    Host {
                        name: "host1".to_string(),
                        cpus: vec![Cpu {
                            name: "cpu1".to_string(),
                            resources: vec![Resource {
                                id: 101,
                                state: ResourceState::Alive,
                                thread_count: 4,
                            }],
                            core_count: 2,
                            cpufreq: 2.4,
                            chassis: "rack1".to_string(),
                            resource_ids: vec![101],
                        }],
                        network_address: "10.0.0.1".to_string(),
                        resource_ids: vec![101],
                        state: ResourceState::Alive,
                    },
                    // Host 2 with resource 102
                    Host {
                        name: "host2".to_string(),
                        cpus: vec![Cpu {
                            name: "cpu2".to_string(),
                            resources: vec![Resource {
                                id: 102,
                                state: ResourceState::Alive,
                                thread_count: 8,
                            }],
                            core_count: 4,
                            cpufreq: 3.2,
                            chassis: "rack1".to_string(),
                            resource_ids: vec![102],
                        }],
                        network_address: "10.0.0.2".to_string(),
                        resource_ids: vec![102],
                        state: ResourceState::Alive,
                    },
                ],
                resource_ids: vec![101, 102],
                state: ResourceState::Alive,
            },
            // Cluster 2 with resources 103, 104
            Cluster {
                name: "cluster2".to_string(),
                hosts: vec![
                    // Host 3 with resource 103
                    Host {
                        name: "host3".to_string(),
                        cpus: vec![Cpu {
                            name: "cpu3".to_string(),
                            resources: vec![Resource {
                                id: 103,
                                state: ResourceState::Alive,
                                thread_count: 4,
                            }],
                            core_count: 2,
                            cpufreq: 2.8,
                            chassis: "rack2".to_string(),
                            resource_ids: vec![103],
                        }],
                        network_address: "10.0.0.3".to_string(),
                        resource_ids: vec![103],
                        state: ResourceState::Alive,
                    },
                    // Host 4 with resource 104
                    Host {
                        name: "host4".to_string(),
                        cpus: vec![Cpu {
                            name: "cpu4".to_string(),
                            resources: vec![Resource {
                                id: 104,
                                state: ResourceState::Alive,
                                thread_count: 4,
                            }],
                            core_count: 2,
                            cpufreq: 2.8,
                            chassis: "rack2".to_string(),
                            resource_ids: vec![104],
                        }],
                        network_address: "10.0.0.4".to_string(),
                        resource_ids: vec![104],
                        state: ResourceState::Alive,
                    },
                ],
                resource_ids: vec![103, 104],
                state: ResourceState::Alive,
            },
        ]
    }

    // Helper function to create and populate an ApplicationContext with test data
    fn setup_test_app_context() -> ApplicationContext {
        let mut context = ApplicationContext::default();

        // Add test jobs and clusters
        context.all_jobs = create_test_jobs();
        context.all_clusters = create_test_clusters();

        // Jobs clusters and hosts are already populated in create_test_jobs
        // This is normally done by the application when loading jobs

        // Initially, filtered_jobs should contain all jobs
        context.filtered_jobs = context.all_jobs.clone();

        context
    }

    #[test]
    /// Tests filtering jobs by owner
    fn test_filter_by_owner() {
        let mut context = setup_test_app_context();

        // Filter by owner "user1"
        context.filters.set_owners(Some(vec!["user1".to_string()]));
        context.filter_jobs();

        assert_eq!(
            context.filtered_jobs.len(),
            2,
            "Should have 2 jobs from user1"
        );
        assert!(
            context.filtered_jobs.iter().all(|j| j.owner == "user1"),
            "All filtered jobs should be owned by user1"
        );

        // Filter by owner "user2"
        context.filters.set_owners(Some(vec!["user2".to_string()]));
        context.filter_jobs();

        assert_eq!(
            context.filtered_jobs.len(),
            1,
            "Should have 1 job from user2"
        );
        assert_eq!(
            context.filtered_jobs[0].owner, "user2",
            "The job should be owned by user2"
        );

        // Filter by multiple owners
        context
            .filters
            .set_owners(Some(vec!["user1".to_string(), "user2".to_string()]));
        context.filter_jobs();

        assert_eq!(context.filtered_jobs.len(), 3, "Should have all 3 jobs");

        // Reset owner filter
        context.filters.set_owners(None);
        context.filter_jobs();

        assert_eq!(
            context.filtered_jobs.len(),
            3,
            "Should have all 3 jobs when owner filter is None"
        );
    }

    #[test]
    /// Tests filtering jobs by state
    fn test_filter_by_state() {
        let mut context = setup_test_app_context();

        // Filter by Running state
        context.filters.set_states(Some(vec![JobState::Running]));
        context.filter_jobs();

        assert_eq!(context.filtered_jobs.len(), 1, "Should have 1 Running job");
        assert_eq!(
            context.filtered_jobs[0].state,
            JobState::Running,
            "The job should be in Running state"
        );

        // Filter by multiple states
        context
            .filters
            .set_states(Some(vec![JobState::Running, JobState::Waiting]));
        context.filter_jobs();

        assert_eq!(
            context.filtered_jobs.len(),
            2,
            "Should have 2 jobs with Running or Waiting states"
        );
        assert!(
            context
                .filtered_jobs
                .iter()
                .all(|j| j.state == JobState::Running || j.state == JobState::Waiting),
            "All filtered jobs should be Running or Waiting"
        );

        // Reset state filter
        context.filters.set_states(None);
        context.filter_jobs();

        assert_eq!(
            context.filtered_jobs.len(),
            3,
            "Should have all 3 jobs when state filter is None"
        );
    }

    #[test]
    /// Tests filtering jobs by time range
    fn test_filter_by_time_range() {
        let mut context = setup_test_app_context();

        // Filter by start time after Jan 1, 2023
        context.filters.scheduled_start_time = Some(1672527600);
        context.filter_jobs();

        assert_eq!(
            context.filtered_jobs.len(),
            3,
            "Should have 3 jobs starting after Jan 1"
        );

        // Filter by start time after Jan 2, 2023
        context.filters.scheduled_start_time = Some(1672614000);
        context.filter_jobs();

        assert_eq!(
            context.filtered_jobs.len(),
            3,
            "Toutes les tâches doivent rester avec le début après Jan 2"
        );

        // Filter by end time before Jan 3, 2023 + 1 hour
        context.filters.wall_time = Some(1672704000); // Jan 3 + 1 hour
        context.filter_jobs();

        // Correction: only 2 jobs should remain (jobs 2 and 3)
        assert_eq!(
            context.filtered_jobs.len(),
            2,
            "Only tasks within the time range should remain"
        );

        // Reset time filters
        context.filters.scheduled_start_time = None;
        context.filters.wall_time = None;
        context.filter_jobs();

        assert_eq!(
            context.filtered_jobs.len(),
            3,
            "Should have all 3 jobs when time filters are None"
        );
    }

    #[test]
    /// Tests filtering jobs by cluster
    fn test_filter_by_cluster() {
        let mut context = setup_test_app_context();

        // Créer un objet cluster avec la hiérarchie complète jusqu'aux ressources
        let cluster1_filter = Cluster {
            name: "cluster1".to_string(),
            hosts: vec![Host {
                name: "host1".to_string(),
                cpus: vec![Cpu {
                    name: "cpu1".to_string(),
                    resources: vec![Resource {
                        id: 101,
                        state: ResourceState::Unknown,
                        thread_count: 0,
                    }],
                    core_count: 0,
                    cpufreq: 0.0,
                    chassis: String::new(),
                    resource_ids: vec![101],
                }],
                network_address: String::new(),
                resource_ids: vec![101],
                state: ResourceState::Unknown,
            }],
            resource_ids: vec![101],
            state: ResourceState::Unknown,
        };

        // Filtrer par cluster1
        context.filters.clusters = Some(vec![cluster1_filter]);
        context.filter_jobs();

        assert_eq!(
            context.filtered_jobs.len(),
            1,
            "Should have 1 job from cluster1"
        );
        assert_eq!(context.filtered_jobs[0].id, 1001, "Job ID should be 1001");

        // Reset cluster filter
        context.filters.clusters = None;
        context.filter_jobs();

        assert_eq!(
            context.filtered_jobs.len(),
            3,
            "Should have all 3 jobs when cluster filter is None"
        );
    }

    #[test]
    /// Tests filtering jobs by host
    fn test_filter_by_host() {
        let mut context = setup_test_app_context();

        // Créer un objet cluster contenant un hôte avec la hiérarchie complète
        let cluster1_filter = Cluster {
            name: "cluster1".to_string(),
            hosts: vec![Host {
                name: "host1".to_string(),
                cpus: vec![Cpu {
                    name: "cpu1".to_string(),
                    resources: vec![Resource {
                        id: 101,
                        state: ResourceState::Unknown,
                        thread_count: 0,
                    }],
                    core_count: 0,
                    cpufreq: 0.0,
                    chassis: String::new(),
                    resource_ids: vec![101],
                }],
                network_address: String::new(),
                resource_ids: vec![101],
                state: ResourceState::Unknown,
            }],
            resource_ids: vec![],
            state: ResourceState::Unknown,
        };

        // Filtrer par host1
        context.filters.clusters = Some(vec![cluster1_filter]);
        context.filter_jobs();

        assert_eq!(
            context.filtered_jobs.len(),
            1,
            "Should have 1 job using host1"
        );
        assert_eq!(context.filtered_jobs[0].id, 1001, "Job ID should be 1001");

        // Reset host filter
        context.filters.clusters = None;
        context.filter_jobs();

        assert_eq!(
            context.filtered_jobs.len(),
            3,
            "Should have all 3 jobs when host filter is None"
        );
    }

    #[test]
    /// Tests combining multiple filter types
    fn test_combined_filters() {
        let mut context = setup_test_app_context();

        // Filter by user1 AND Running state
        context.filters.set_owners(Some(vec!["user1".to_string()]));
        context.filters.set_states(Some(vec![JobState::Running]));
        context.filter_jobs();

        assert_eq!(
            context.filtered_jobs.len(),
            1,
            "Should have 1 job from user1 in Running state"
        );
        assert_eq!(context.filtered_jobs[0].id, 1001, "Job ID should be 1001");

        // Filter by user1 AND (Running OR Terminated) states
        context
            .filters
            .set_states(Some(vec![JobState::Running, JobState::Terminated]));
        context.filter_jobs();

        assert_eq!(
            context.filtered_jobs.len(),
            2,
            "Should have 2 jobs from user1 in Running or Terminated states"
        );

        // Filter by user1 AND (Running OR Terminated) AND after Jan 2
        context.filters.scheduled_start_time = Some(1672614000); // Jan 2
        context.filter_jobs();

        assert_eq!(
            context.filtered_jobs.len(),
            2,
            "Les deux tâches de user1 doivent rester avec le filtre de date"
        );

        // Reset all filters
        context.filters = JobFilters::default();
        context.filter_jobs();

        assert_eq!(
            context.filtered_jobs.len(),
            3,
            "Should have all 3 jobs when all filters are None"
        );
    }
}
