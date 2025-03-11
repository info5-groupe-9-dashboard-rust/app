#[cfg(test)]
mod tests {
    use egui::Color32;

    use crate::models::data_structure::cluster::Cluster;
    use crate::models::data_structure::cpu::Cpu;
    use crate::models::data_structure::host::Host;
    use crate::models::data_structure::job::Job;
    use crate::models::data_structure::resource::{Resource, ResourceState};
    use crate::models::utils::utils::*;
    use std::cmp::Ordering;

    // Helper function to create test clusters
    fn create_test_clusters() -> Vec<Cluster> {
        vec![
            Cluster {
                name: "cluster1".to_string(),
                hosts: vec![
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
            Cluster {
                name: "cluster2".to_string(),
                hosts: vec![Host {
                    name: "host3".to_string(),
                    cpus: vec![Cpu {
                        name: "cpu3".to_string(),
                        resources: vec![Resource {
                            id: 103,
                            state: ResourceState::Dead,
                            thread_count: 4,
                        }],
                        core_count: 2,
                        cpufreq: 2.8,
                        chassis: "rack2".to_string(),
                        resource_ids: vec![103],
                    }],
                    network_address: "10.0.0.3".to_string(),
                    resource_ids: vec![103],
                    state: ResourceState::Dead,
                }],
                resource_ids: vec![103],
                state: ResourceState::Dead,
            },
        ]
    }

    // Helper function to create a test job
    fn create_test_job() -> Job {
        Job {
            id: 1234,
            owner: "test_user".to_string(),
            state: crate::models::data_structure::job::JobState::Running,
            walltime: 3600,
            start_time: 1672527600,
            scheduled_start: 1672527600,
            assigned_resources: vec![101, 102],
            main_resource_state: ResourceState::Alive,
            clusters: Vec::new(),
            hosts: Vec::new(),

            // Champs manquants
            command: "test_command".to_string(),
            message: Some("test_message".to_string()),
            queue: "test_queue".to_string(),
            submission_time: 1672527500, // Juste avant start_time
            stop_time: 0,                // 0 pour un job encore en cours d'exécution
            exit_code: Some(0),          // Code de sortie 0 (succès)
            gantt_color: Color32::from_rgb(0, 0, 0), // Replace with a default color
        }
    }

    #[test]
    /// Tests that color conversion produces consistent results
    fn test_convert_id_to_color() {
        // Test that same ID always gives same color
        let color1 = convert_id_to_color(1234);
        let color2 = convert_id_to_color(1234);
        assert_eq!(color1, color2, "Same ID should produce same color");

        // Test that different IDs produce different colors
        let color3 = convert_id_to_color(5678);
        assert_ne!(
            color1, color3,
            "Different IDs should produce different colors"
        );
    }

    #[test]
    /// Tests extraction of all host names from clusters
    fn test_get_all_hosts() {
        let clusters = create_test_clusters();
        let hosts = get_all_hosts(&clusters);

        assert_eq!(hosts.len(), 3, "Should find 3 hosts");
        assert!(hosts.contains(&"host1".to_string()));
        assert!(hosts.contains(&"host2".to_string()));
        assert!(hosts.contains(&"host3".to_string()));
    }

    #[test]
    /// Tests checking if a cluster contains a specific host
    fn test_cluster_contain_host() {
        let clusters = create_test_clusters();
        let cluster1 = &clusters[0];

        assert!(
            cluster_contain_host(cluster1, "host1"),
            "Cluster1 should contain host1"
        );
        assert!(
            cluster_contain_host(cluster1, "host2"),
            "Cluster1 should contain host2"
        );
        assert!(
            !cluster_contain_host(cluster1, "host3"),
            "Cluster1 should not contain host3"
        );
    }

    #[test]
    /// Tests retrieving a cluster by name
    fn test_get_cluster_from_name() {
        let clusters = create_test_clusters();

        let result1 = get_cluster_from_name(&clusters, "cluster1");
        assert!(result1.is_some(), "Should find cluster1");
        assert_eq!(result1.unwrap().name, "cluster1");

        let result2 = get_cluster_from_name(&clusters, "nonexistent");
        assert!(result2.is_none(), "Should not find nonexistent cluster");
    }

    #[test]
    /// Tests extraction of all cluster names
    fn test_get_all_clusters() {
        let clusters = create_test_clusters();
        let cluster_names = get_all_clusters(&clusters);

        assert_eq!(cluster_names.len(), 2, "Should find 2 clusters");
        assert!(cluster_names.contains(&"cluster1".to_string()));
        assert!(cluster_names.contains(&"cluster2".to_string()));
    }

    #[test]
    /// Tests extraction of all resource IDs
    fn test_get_all_resources() {
        let clusters = create_test_clusters();
        let resources = get_all_resources(&clusters);

        assert_eq!(resources.len(), 3, "Should find 3 resources");
        assert!(resources.contains(&101));
        assert!(resources.contains(&102));
        assert!(resources.contains(&103));
    }

    #[test]
    /// Tests checking if any cluster contains a specific host
    fn test_contains_host() {
        let clusters = create_test_clusters();

        assert!(
            contains_host(&clusters, "host1"),
            "Clusters should contain host1"
        );
        assert!(
            contains_host(&clusters, "host3"),
            "Clusters should contain host3"
        );
        assert!(
            !contains_host(&clusters, "host4"),
            "Clusters should not contain host4"
        );
    }

    #[test]
    /// Tests getting a cluster's state by name
    fn test_get_cluster_state_from_name() {
        let clusters = create_test_clusters();

        let state1 = get_cluster_state_from_name(&clusters, &"cluster1".to_string());
        assert_eq!(state1, ResourceState::Alive, "cluster1 should be Alive");

        let state2 = get_cluster_state_from_name(&clusters, &"cluster2".to_string());
        assert_eq!(state2, ResourceState::Dead, "cluster2 should be Dead");

        let state3 = get_cluster_state_from_name(&clusters, &"nonexistent".to_string());
        assert_eq!(
            state3,
            ResourceState::Unknown,
            "Nonexistent cluster should return Unknown state"
        );
    }

    #[test]
    /// Tests getting a host's state by name
    fn test_get_host_state_from_name() {
        let clusters = create_test_clusters();

        let state1 = get_host_state_from_name(&clusters, &"host1".to_string());
        assert_eq!(state1, ResourceState::Alive, "host1 should be Alive");

        let state2 = get_host_state_from_name(&clusters, &"host3".to_string());
        assert_eq!(state2, ResourceState::Dead, "host3 should be Dead");

        let state3 = get_host_state_from_name(&clusters, &"nonexistent".to_string());
        assert_eq!(
            state3,
            ResourceState::Unknown,
            "Nonexistent host should return Unknown state"
        );
    }

    #[test]
    /// Tests checking if a cluster with specific name exists
    fn test_contains_cluster() {
        let clusters = create_test_clusters();

        assert!(
            contains_cluster(&clusters, "cluster1"),
            "Should contain cluster1"
        );
        assert!(
            contains_cluster(&clusters, "cluster2"),
            "Should contain cluster2"
        );
        assert!(
            !contains_cluster(&clusters, "cluster3"),
            "Should not contain cluster3"
        );
    }

    #[test]
    /// Tests natural string comparison with embedded numbers
    fn test_compare_string_with_number() {
        // Test basic alphabetical comparison
        assert_eq!(compare_string_with_number("abc", "def"), Ordering::Less);
        assert_eq!(compare_string_with_number("def", "abc"), Ordering::Greater);
        assert_eq!(compare_string_with_number("abc", "abc"), Ordering::Equal);

        // Test numeric comparison within strings (natural sort)
        assert_eq!(compare_string_with_number("host1", "host2"), Ordering::Less);
        assert_eq!(
            compare_string_with_number("host2", "host10"),
            Ordering::Less,
            "Natural sort: host2 should come before host10"
        );
        assert_eq!(
            compare_string_with_number("host10", "host2"),
            Ordering::Greater,
            "Natural sort: host10 should come after host2"
        );

        // Test mixed strings
        assert_eq!(
            compare_string_with_number("abc123", "abc45"),
            Ordering::Greater,
            "123 > 45 in natural sort"
        );
    }

    #[test]
    /// Tests finding clusters related to a job
    fn test_get_clusters_for_job() {
        let clusters = create_test_clusters();
        let job = create_test_job();

        let job_clusters = get_clusters_for_job(&job, &clusters);

        assert_eq!(job_clusters.len(), 1, "Job should be related to 1 cluster");
        assert_eq!(
            job_clusters[0], "cluster1",
            "Job should be related to cluster1"
        );
    }

    #[test]
    /// Tests finding hosts related to a job
    fn test_get_hosts_for_job() {
        let clusters = create_test_clusters();
        let job = create_test_job();

        let job_hosts = get_hosts_for_job(&job, &clusters);

        assert_eq!(job_hosts.len(), 2, "Job should be related to 2 hosts");
        assert!(
            job_hosts.contains(&"host1".to_string()),
            "Job should use host1"
        );
        assert!(
            job_hosts.contains(&"host2".to_string()),
            "Job should use host2"
        );
    }

    #[test]
    /// Tests creating a tree structure for a specific job
    fn test_get_tree_structure_for_job() {
        let clusters = create_test_clusters();
        let job = create_test_job();

        let job_tree = get_tree_structure_for_job(&job, &clusters);

        // Verify structure
        assert_eq!(job_tree.len(), 1, "Should return 1 cluster");
        assert_eq!(job_tree[0].name, "cluster1", "Should be cluster1");
        assert_eq!(job_tree[0].hosts.len(), 2, "Should have 2 hosts");

        // Check that only relevant resources are included
        for cluster in &job_tree {
            for host in &cluster.hosts {
                for cpu in &host.cpus {
                    for resource in &cpu.resources {
                        assert!(
                            job.assigned_resources.contains(&resource.id),
                            "Resource {} should belong to job",
                            resource.id
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn test_natural_sort() {
        assert_eq!(
            compare_string_with_number("abc123", "abc45"),
            Ordering::Greater
        );

        assert_eq!(
            compare_string_with_number("img10.png", "img2.png"),
            Ordering::Greater
        );
        assert_eq!(
            compare_string_with_number("file99.txt", "file100.txt"),
            Ordering::Less
        );
    }
}
