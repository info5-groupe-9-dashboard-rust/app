use crate::models::data_structure::cluster::Cluster;
use crate::models::data_structure::cpu::Cpu;
use crate::models::data_structure::host::Host;
use crate::models::data_structure::job::Job;
use crate::models::data_structure::resource::ResourceState;
use std::hash::DefaultHasher;
use std::hash::Hash;
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
