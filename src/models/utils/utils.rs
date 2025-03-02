use crate::models::data_structure::cluster::Cluster;
use crate::models::data_structure::cpu::Cpu;
use crate::models::data_structure::host::Host;
use crate::models::data_structure::job::Job;
use crate::models::data_structure::resource::ResourceState;
use std::hash::DefaultHasher;
use std::hash::Hash;
//
use std::cmp::Ordering;
use std::hash::Hasher;

// Convert a job ID to a color (using hash)
pub fn convert_id_to_color(id: u32) -> egui::Color32 {
    let mut hasher = DefaultHasher::new();
    id.hash(&mut hasher);
    let hash = hasher.finish();

    let r = ((hash >> 16) & 0xFF) as u8;
    let g = ((hash >> 8) & 0xFF) as u8;
    let b = (hash & 0xFF) as u8;

    egui::Color32::from_rgb(r, g, b)
}

pub fn get_all_hosts(clusters: &Vec<Cluster>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for cluster in clusters {
        for host in &cluster.hosts {
            result.push(host.name.clone());
        }
    }

    result
}

pub fn cluster_contain_host(cluster: &Cluster, host_name: &str) -> bool {
    for host in &cluster.hosts {
        if host.name == host_name {
            return true;
        }
    }
    false
}

pub fn get_cluster_from_name(clusters: &Vec<Cluster>, name: &str) -> Option<Cluster> {
    for cluster in clusters {
        if cluster.name == name {
            return Some(cluster.clone());
        }
    }
    None
}

pub fn get_all_clusters(clusters: &Vec<Cluster>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for cluster in clusters {
        result.push(cluster.name.clone());
    }

    result
}

pub fn get_all_resources(clusters: &Vec<Cluster>) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    for cluster in clusters {
        result.extend(cluster.resource_ids.iter());
    }

    result
}

pub fn contains_host(cluster: &Vec<Cluster>, host_name: &str) -> bool {
    for c in cluster {
        for host in &c.hosts {
            if host.name == host_name {
                return true;
            }
        }
    }
    false
}

pub fn get_cluster_state_from_name(cluster: &Vec<Cluster>, cluster_name: &String) -> ResourceState {
    for c in cluster {
        if c.name == *cluster_name {
            return c.state.clone();
        }
    }
    ResourceState::Unknown
}

pub fn get_host_state_from_name(cluster: &Vec<Cluster>, host_name: &String) -> ResourceState {
    for c in cluster {
        for host in &c.hosts {
            if host.name == *host_name {
                return host.state.clone();
            }
        }
    }
    ResourceState::Unknown
}

pub fn contains_cluster(cluster: &Vec<Cluster>, cluster_name: &str) -> bool {
    for c in cluster {
        if c.name == cluster_name {
            return true;
        }
    }
    false
}

// Compare two strings that may contain numbers
pub fn compare_string_with_number(a: &str, b: &str) -> Ordering {
    let mut strings_a: Vec<String> = Vec::new();
    let mut strings_b: Vec<String> = Vec::new();
    let mut int_a: Vec<i32> = Vec::new();
    let mut int_b: Vec<i32> = Vec::new();
    let mut order_a: Vec<String> = Vec::new();
    let mut order_b: Vec<String> = Vec::new();

    let mut curr = 0;

    let mut string_count = 0;

    if !a.chars().next().unwrap().is_numeric() {
        order_a.push("string".to_string());
        strings_a.push(String::new());
    }

    for c in a.chars() {
        if c.is_numeric() {
            curr = curr * 10 + c.to_digit(10).unwrap() as i32;
        } else if curr == 0 {
            strings_a[string_count].push(c);
        } else {
            strings_a.push(String::new());
            string_count += 1;
            strings_a[string_count].push(c);
            int_a.push(curr);
            order_a.push("int".to_string());
            order_a.push("string".to_string());
            curr = 0;
        }
    }

    curr = 0;
    string_count = 0;

    if !b.chars().next().unwrap().is_numeric() {
        order_b.push("string".to_string());
        strings_b.push(String::new());
    }

    for c in b.chars() {
        if c.is_numeric() {
            curr = curr * 10 + c.to_digit(10).unwrap() as i32;
        } else if curr == 0 {
            strings_b[string_count].push(c);
        } else {
            strings_b.push(String::new());
            string_count += 1;
            strings_b[string_count].push(c);
            int_b.push(curr);
            order_b.push("int".to_string());
            order_b.push("string".to_string());
            curr = 0;
        }
    }

    // Once the three vectors are created, we can compare them
    // the comparison will be done in the same order as the order vector
    // if the order is the same, we will compare the strings

    let mut index_int = 0;
    let mut index_string = 0;

    for i in 0..order_a.len() {
        if order_a[i] == "int" && order_b[i] == "int" {
            if int_a[index_int] < int_b[index_int] {
                return Ordering::Less;
            } else if int_a[index_int] > int_b[index_int] {
                return Ordering::Greater;
            }
            index_int += 1;
        } else if order_a[i] == "string" && order_b[i] == "string" {
            if strings_a[index_string] < strings_b[index_string] {
                return Ordering::Less;
            } else if strings_a[index_string] > strings_b[index_string] {
                return Ordering::Greater;
            }
            index_string += 1;
        } else if order_a[i] == "int" && order_b[i] == "string" {
            return Ordering::Less;
        } else if order_a[i] == "string" && order_b[i] == "int" {
            return Ordering::Greater;
        }
    }

    if order_a.len() < order_b.len() {
        return Ordering::Less;
    } else if order_a.len() > order_b.len() {
        return Ordering::Greater;
    }

    return Ordering::Equal;
}

// Return the name of all the clusters where the job is running
pub fn get_clusters_for_job(job: &Job, clusters: &Vec<Cluster>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for resource in &job.assigned_resources {
        for cluster in clusters {
            if !result.contains(&cluster.name) && cluster.resource_ids.contains(&resource) {
                result.push(cluster.name.clone());
            }
        }
    }

    result
}

pub fn get_hosts_for_job(job: &Job, clusters: &Vec<Cluster>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for resource in &job.assigned_resources {
        for cluster in clusters {
            if cluster.resource_ids.contains(&resource) {
                for host in &cluster.hosts {
                    if !result.contains(&host.name) && host.resource_ids.contains(&resource) {
                        result.push(host.name.clone());
                    }
                }
            }
        }
    }

    result
}

pub fn get_tree_structure_for_job(job: &Job, clusters: &Vec<Cluster>) -> Vec<Cluster> {
    let mut result: Vec<Cluster> = Vec::new();

    for resource in &job.assigned_resources {
        for cluster in clusters {
            if cluster.resource_ids.contains(&resource) {
                // if the cluster already exists in the result, add the resource to the resource_ids
                if let Some(existing_cluster) = result.iter_mut().find(|c| c.name == cluster.name) {
                    existing_cluster.resource_ids.push(*resource);
                    // Then check if in the existing cluster the host already exists
                    for host in &cluster.hosts {
                        if host.resource_ids.contains(&resource) {
                            if let Some(existing_host) = existing_cluster
                                .hosts
                                .iter_mut()
                                .find(|h| h.name == host.name)
                            {
                                existing_host.resource_ids.push(*resource);
                                // Then check if in the existing host the CPU already exists
                                for cpu in &host.cpus {
                                    if cpu.resource_ids.contains(&resource) {
                                        if let Some(existing_cpu) = existing_host
                                            .cpus
                                            .iter_mut()
                                            .find(|c| c.name == cpu.name)
                                        {
                                            existing_cpu.resource_ids.push(*resource);
                                            // Then check if in the existing CPU the resource already exists
                                            if let Some(existing_res) = existing_cpu
                                                .resources
                                                .iter_mut()
                                                .find(|res| res.id == *resource)
                                            {
                                                existing_res.thread_count += 1;
                                            } else {
                                                let res = cpu
                                                    .resources
                                                    .iter()
                                                    .find(|res| res.id == *resource);

                                                if let Some(res) = res {
                                                    existing_cpu.resources.push(res.clone());
                                                }
                                            }
                                        } else {
                                            let mut new_cpu = Cpu {
                                                name: cpu.name.clone(),
                                                resource_ids: vec![*resource],
                                                chassis: cpu.chassis.clone(),
                                                core_count: cpu.core_count,
                                                cpufreq: cpu.cpufreq,
                                                resources: Vec::new(),
                                            };

                                            let res = cpu
                                                .resources
                                                .iter()
                                                .find(|res| res.id == *resource);

                                            if let Some(res) = res {
                                                new_cpu.resources.push(res.clone());
                                            }

                                            existing_host.cpus.push(new_cpu);
                                        }
                                    }
                                }
                            } else {
                                let mut new_host = Host {
                                    name: host.name.clone(),
                                    resource_ids: vec![*resource],
                                    cpus: Vec::new(),
                                    network_address: host.network_address.clone(),
                                    state: ResourceState::Unknown,
                                };

                                for cpu in &host.cpus {
                                    let mut new_cpu = Cpu {
                                        name: cpu.name.clone(),
                                        resource_ids: vec![*resource],
                                        chassis: cpu.chassis.clone(),
                                        core_count: cpu.core_count,
                                        cpufreq: cpu.cpufreq,
                                        resources: Vec::new(),
                                    };

                                    let res = cpu.resources.iter().find(|res| res.id == *resource);

                                    if let Some(res) = res {
                                        new_cpu.resources.push(res.clone());
                                    }

                                    new_host.cpus.push(new_cpu);
                                }

                                existing_cluster.hosts.push(new_host);
                            }
                        }
                    }
                } else {
                    let new_cluster = Cluster {
                        name: cluster.name.clone(),
                        // new resource_ids with only the current resource
                        resource_ids: vec![*resource],

                        // for the host only keep the hosts that the job is running on
                        hosts: cluster
                            .hosts
                            .iter()
                            .filter(|host| host.resource_ids.contains(&resource))
                            .map(|host| {
                                let mut new_host = Host {
                                    name: host.name.clone(),
                                    resource_ids: vec![*resource],
                                    cpus: Vec::new(),
                                    network_address: host.network_address.clone(),
                                    state: ResourceState::Unknown,
                                };

                                // for the CPU only keep the CPUs that the job is running on
                                new_host.cpus = host
                                    .cpus
                                    .iter()
                                    .filter(|cpu| cpu.resource_ids.contains(&resource))
                                    .map(|cpu| {
                                        let mut new_cpu = Cpu {
                                            name: cpu.name.clone(),
                                            resource_ids: vec![*resource],
                                            chassis: cpu.chassis.clone(),
                                            core_count: cpu.core_count,
                                            cpufreq: cpu.cpufreq,
                                            // get the current resource from the CPU resources and create a new vector with only that resource
                                            resources: Vec::new(),
                                        };

                                        let res =
                                            cpu.resources.iter().find(|res| res.id == *resource);

                                        if let Some(res) = res {
                                            new_cpu.resources.push(res.clone());
                                        }

                                        new_cpu
                                    })
                                    .collect();

                                new_host
                            })
                            .collect(),
                        state: ResourceState::Unknown,
                    };

                    result.push(new_cluster);
                }
            }
        }
    }
    result
}
