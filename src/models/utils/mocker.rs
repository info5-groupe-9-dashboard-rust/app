use chrono::Local;

use crate::models::data_structure::{job::{Job, JobState}, strata::Strata, resource::ResourceState};

// Mocking Job
fn mock_job(id: u32) -> Job {
    // Possible owner list
    let owners = vec!["alice", "bob", "charlie", "david", "eva"];

    // List of possible commands and their queues associated
    let commands = vec![
        ("python3 train_model.py", "gpu"),
        ("make test", "test"),
        ("mpirun -np 4 simulation", "cpu"),
        ("jupyter notebook", "interactive"),
        ("gcc -O3 project.c", "compile"),
    ];

    // Possible clusters list
    let clusters_list = vec!["cluster1", "cluster2", "cluster3"];
    
    // Possible hosts list
    let hosts_list = vec!["host1", "host2", "host3", "host4"];

    // Function to generate a random number
    let random_index = |max: usize| -> usize {
        let mut buf = [0u8; 8];
        getrandom::getrandom(&mut buf).unwrap();
        let value = u64::from_le_bytes(buf);
        (value % max as u64) as usize
    };

    let random_float = || -> f32 {
        let mut buf = [0u8; 8];
        getrandom::getrandom(&mut buf).unwrap();
        let value = u64::from_le_bytes(buf);
        (value as f32) / (u64::MAX as f32)
    };

    // Coherent timestamp generation
    let now = Local::now().timestamp();
    let submission_time = now - (random_index(86400) as i64);
    let scheduled_start = submission_time + (random_index(3300) as i64 + 300);
    let start_time = if random_float() < 0.7 {
        scheduled_start
    } else {
        0
    };
    let walltime = random_index(5400) as i64 + 1800;
    let stop_time = if start_time > 0 && random_float() < 0.3 {
        start_time + walltime
    } else {
        0
    };

    // Randomly select the state based on the context
    let state = if stop_time > 0 {
        JobState::Terminated
    } else if start_time > 0 {
        let states = vec![JobState::Running, JobState::Suspended, JobState::Finishing];
        states[random_index(states.len())].clone()
    } else if scheduled_start > now {
        let states = vec![JobState::Waiting, JobState::Hold];
        states[random_index(states.len())].clone()
    } else {
        let states = vec![JobState::ToLaunch, JobState::Launching, JobState::Waiting];
        states[random_index(states.len())].clone()
    };

    // Command and queue selection
    let (command, queue) = commands[random_index(commands.len())];

    // Assigned ressources generation
    let num_resources = random_index(7) + 1;
    let assigned_resources = if start_time > 0 {
        let mut resources = Vec::new();
        while resources.len() < num_resources {
            let resource = (random_index(20) + 1) as u32;
            if !resources.contains(&resource) {
                resources.push(resource);
            }
        }
        resources
    } else {
        vec![]
    };

    // Generate message based on the state
    let message = match state {
        JobState::Error => Some("Erreur d'exécution".to_string()),
        JobState::Hold => Some("En attente de ressources".to_string()),
        JobState::Suspended => Some("Suspendu par l'administrateur".to_string()),
        _ => None,
    };

    // Generate random clusters
    let num_clusters = random_index(2) + 1;
    let clusters = clusters_list.iter()
        .take(num_clusters)
        .map(|&c| c.to_string())
        .collect();

    // Generate random hosts
    let num_hosts = random_index(3) + 1;
    let hosts = hosts_list.iter()
        .take(num_hosts)
        .map(|&h| h.to_string())
        .collect();

    // Generate gantt color
    let gantt_color = egui::Color32::from_rgb(
        random_index(255) as u8,
        random_index(255) as u8,
        random_index(255) as u8,
    );

    // Generate main resource state
    let main_resource_state = match random_index(3) {
        0 => ResourceState::Alive,
        1 => ResourceState::Dead,
        _ => ResourceState::Absent,
    };

    Job {
        id,
        owner: owners[random_index(owners.len())].to_string(),
        state,
        command: command.to_string(),
        walltime,
        message,
        queue: queue.to_string(),
        assigned_resources,
        scheduled_start,
        submission_time,
        start_time,
        stop_time,
        exit_code: if stop_time > 0 {
            Some((random_index(3) as i32) - 1)
        } else {
            None
        },
        gantt_color,
        clusters,
        hosts,
        main_resource_state,
    }
}

pub fn mock_jobs() -> Vec<Job> {
    (1..=50).map(|id| mock_job(id)).collect()
}

fn mock_strata(id: u32) -> Strata {
    // Possible cluster list
    let clusters_list = vec!["cluster1", "cluster2", "cluster3", "cluster4", "cluster5"];
    
    // Possible hosts list
    let hosts_list = vec!["host1", "host2", "host3", "host4", "host5", "host6"];

    // Possible states list
    let states_list = vec!["Dead", "Alive", "Absent", "Unknown"];

    // Possible comments list
    let comments_list = vec!["No issues", "Minor issues", "Major issues", "Critical issues"];

    // Function to generate a random number
    let random_index = |max: usize| -> usize {
        let mut buf = [0u8; 8];
        getrandom::getrandom(&mut buf).unwrap();
        let value = u64::from_le_bytes(buf);
        (value % max as u64) as usize
    };

    let random_float = || -> f32 {
        let mut buf = [0u8; 8];
        getrandom::getrandom(&mut buf).unwrap();
        let value = u64::from_le_bytes(buf);
        (value as f32) / (u64::MAX as f32)
    };

    Strata {
        state_num: Some(random_index(10) as i32),
        thread_count: Some(random_index(16) as i32),
        rconsole: Some(format!("rconsole{}", random_index(100))),
        memnode: Some(random_index(64000) as i64),
        cluster: Some(clusters_list[random_index(clusters_list.len())].to_string()),
        desktop_computing: Some(if random_index(2) == 0 { "enabled" } else { "disabled" }.to_string()),
        memcore: Some(random_index(128) as i32),
        production: Some(if random_index(2) == 0 { "production" } else { "development" }.to_string()),
        eth_rate: Some(random_index(1000) as i32),
        chassis: Some(format!("chassis{}", random_index(10))),
        memcpu: Some(random_index(64000) as i64),
        cluster_priority: Some(random_index(10) as i32),
        gpu_model: Some(format!("gpu_model{}", random_index(10))),
        gpu_compute_capability: Some(format!("gpu_compute_capability{}", random_index(10))),
        core_count: Some(random_index(64) as i32),
        next_state: Some(format!("next_state{}", random_index(10))),
        cpufreq: Some(format!("cpufreq{}", random_index(10))),
        comment: Some(comments_list[random_index(comments_list.len())].to_string()),
        core: Some(random_index(64) as i32),
        cpuset: Some(format!("cpuset{}", random_index(10))),
        suspended_jobs: Some(format!("suspended_jobs{}", random_index(10))),
        state: Some(states_list[random_index(states_list.len())].to_string()),
        ip: Some(format!("192.168.{}.{}", random_index(256), random_index(256))),
        network_address: Some(format!("network_address{}", random_index(100))),
        resource_id: Some(id),
        host: Some(hosts_list[random_index(hosts_list.len())].to_string()),
        nodemodel: Some(format!("nodemodel{}", random_index(10))),
        cputype: Some(format!("cputype{}", random_index(10))),
    }
}

pub fn mock_stratas() -> Vec<Strata> {
    (1..=50).map(|id| mock_strata(id)).collect()
}

// Mocking Resource
// fn mock_resource(id: u32) -> Resource {
//     // Function to generate a random number
//     let random_index = |max: usize| -> usize {
//         let mut buf = [0u8; 8];
//         getrandom::getrandom(&mut buf).unwrap();
//         let value = u64::from_le_bytes(buf);
//         (value % max as u64) as usize
//     };

//     // Randomly select the state
//     let state = match random_index(4) {
//         0 => ResourceState::Dead,
//         1 => ResourceState::Alive,
//         2 => ResourceState::Absent,
//         _ => ResourceState::Unknown,
//     };

//     // Generate random thread count
//     let thread_count = random_index(100) as i32;

//     Resource {
//         id,
//         state,
//         thread_count,
//     }
// }

// pub fn mock_resources() -> Vec<Resource> {
//     (1..=50).map(|id| mock_resource(id)).collect()
// }

// // Mocking Cpu
// fn mock_cpu(id: u32) -> Cpu {
//     // Possible names list
//     let names = vec!["Intel Xeon", "AMD Ryzen", "ARM Cortex", "Qualcomm Snapdragon", "Apple M1"];

//     // Possible chassis list
//     let chassis_list = vec!["Chassis1", "Chassis2", "Chassis3"];

//     // Function to generate a random number
//     let random_index = |max: usize| -> usize {
//         let mut buf = [0u8; 8];
//         getrandom::getrandom(&mut buf).unwrap();
//         let value = u64::from_le_bytes(buf);
//         (value % max as u64) as usize
//     };

//     let random_float = || -> f32 {
//         let mut buf = [0u8; 8];
//         getrandom::getrandom(&mut buf).unwrap();
//         let value = u64::from_le_bytes(buf);
//         (value as f32) / (u64::MAX as f32)
//     };

//     // Generate random resources
//     let resources = mock_resources();

//     // Generate random resource ids
//     let resource_ids: Vec<u32> = resources.iter().map(|r| r.id).collect();

//     Cpu {
//         name: names[random_index(names.len())].to_string(),
//         resources,
//         chassis: chassis_list[random_index(chassis_list.len())].to_string(),
//         core_count: random_index(16) as i32 + 1,
//         cpufreq: random_float() * 3.5 + 1.0,
//         resource_ids,
//     }
// }

// pub fn mock_cpus() -> Vec<Cpu> {
//     (1..=50).map(|id| mock_cpu(id)).collect()
// }

// // Mocking Host
// fn mock_host(id: u32) -> Host {
//     // Possible host names
//     let host_names = vec!["host1", "host2", "host3", "host4", "host5"];

//     // Possible network addresses
//     let network_addresses = vec![
//         "192.168.1.1",
//         "192.168.1.2",
//         "192.168.1.3",
//         "192.168.1.4",
//         "192.168.1.5",
//     ];

//     // Function to generate a random number
//     let random_index = |max: usize| -> usize {
//         let mut buf = [0u8; 8];
//         getrandom::getrandom(&mut buf).unwrap();
//         let value = u64::from_le_bytes(buf);
//         (value % max as u64) as usize
//     };

//     // Generate random resource IDs
//     let num_resources = random_index(7) + 1;
//     let resource_ids = (0..num_resources).map(|_| random_index(100) as u32).collect();

//     // Generate random state
//     let state = match random_index(3) {
//         0 => ResourceState::Alive,
//         1 => ResourceState::Dead,
//         _ => ResourceState::Absent,
//     };

//     Host {
//         name: host_names[random_index(host_names.len())].to_string(),
//         cpus: mock_cpus(),
//         network_address: network_addresses[random_index(network_addresses.len())].to_string(),
//         resource_ids,
//         state,
//     }
// }

// pub fn mock_hosts() -> Vec<Host> {
//     (1..=50).map(|id| mock_host(id)).collect()
// }


// fn mock_cluster(name: &str) -> Cluster {
//     // Possible resource states
//     let resource_states = vec![ResourceState::Alive, ResourceState::Dead, ResourceState::Absent];

//     // Function to generate a random number
//     let random_index = |max: usize| -> usize {
//         let mut buf = [0u8; 8];
//         getrandom::getrandom(&mut buf).unwrap();
//         let value = u64::from_le_bytes(buf);
//         (value % max as u64) as usize
//     };

//     // Generate random hosts
//     let hosts = mock_hosts();

//     // Generate random resource ids
//     let num_resources = random_index(10) + 1;
//     let resource_ids = (0..num_resources).map(|_| random_index(100) as u32).collect();

//     // Generate random state
//     let state = resource_states[random_index(resource_states.len())].clone();

//     Cluster {
//         name: name.to_string(),
//         hosts,
//         resource_ids,
//         state,
//     }
// }

// fn mock_clusters() -> Vec<Cluster> {
//     let cluster_names = vec!["cluster1", "cluster2", "cluster3"];
//     cluster_names.iter().map(|&name| mock_cluster(name)).collect()
// }
