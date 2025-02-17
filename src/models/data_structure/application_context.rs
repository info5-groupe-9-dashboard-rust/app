use super::cluster::Cluster;
use super::filters::JobFilters;
use super::job::Job;
use super::resource::Resource;
use super::strata::Strata;
use crate::models::data_structure::cpu::Cpu;
use crate::models::data_structure::host::Host;
use crate::views::components::job_table::JobTable;
use crate::views::view::ViewType;
use chrono::{DateTime, Utc};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

#[cfg(target_arch = "wasm32")]
use crate::models::job::mock_jobs;

pub struct ApplicationContext {
    pub all_jobs: Vec<Job>,
    pub filtered_jobs: Vec<Job>,

    pub all_clusters: Vec<Cluster>,

    pub start_date: Arc<Mutex<DateTime<Utc>>>,
    pub end_date: Arc<Mutex<DateTime<Utc>>>,
    pub view_type: ViewType,
    pub is_loading: bool,
    pub is_refreshing: Arc<Mutex<bool>>,
    pub refresh_rate: Arc<Mutex<u64>>,
    pub filters: JobFilters,
    pub job_table: JobTable,

    pub jobs_receiver: Receiver<Vec<Job>>,
    pub jobs_sender: Sender<Vec<Job>>,
    pub resources_receiver: Receiver<Vec<Strata>>,
    pub resources_sender: Sender<Vec<Strata>>,
}

impl ApplicationContext {
    pub fn check_job_update(&mut self) {
        // Check if new data is available
        if let Ok(new_jobs) = self.jobs_receiver.try_recv() {
            self.all_jobs = new_jobs;
            self.is_loading = false;
        }
        self.filter_jobs();
    }

    pub fn check_ressource_update(&mut self) {
        if let Ok(new_resources) = self.resources_receiver.try_recv() {
            // for every resources get the cluster name with resource.cluster and if there is no cluster with this name in all_clusters add it to all_clusters
            for resource in new_resources.iter() {
                let cluster_name = resource.cluster.as_ref().unwrap_or(&"".to_string()).clone();
                if cluster_name == "" {
                    continue;
                }
                if !self
                    .all_clusters
                    .iter()
                    .any(|cluster| cluster.name == cluster_name)
                {
                    // Add the cluster to all_clusters with one host being resource.host
                    let new_cluster = Cluster {
                        name: cluster_name.clone(),
                        hosts: vec![Host {
                            name: resource.host.as_ref().unwrap_or(&"".to_string()).clone(),
                            cpus: vec![Cpu {
                                name: resource
                                    .nodemodel
                                    .as_ref()
                                    .unwrap_or(&"".to_string())
                                    .clone(),
                                resources: vec![Resource {
                                    id: resource.resource_id.unwrap_or(0) as i32,
                                    state: match resource
                                        .state
                                        .as_ref()
                                        .unwrap_or(&"".to_string())
                                        .as_str()
                                    {
                                        "Dead" => super::resource::ResourceState::Dead,
                                        "Alive" => super::resource::ResourceState::Alive,
                                        "Absent" => super::resource::ResourceState::Absent,
                                        _ => super::resource::ResourceState::Unknown,
                                    },
                                    thread_count: resource.thread_count.unwrap_or(0) as i32,
                                }],
                                core_count: resource.core_count.unwrap_or(0) as i32,
                                cpufreq: resource
                                    .cpufreq
                                    .as_ref()
                                    .unwrap_or(&"".to_string())
                                    .parse::<f32>()
                                    .unwrap_or(0.0),
                                chassis: resource
                                    .chassis
                                    .as_ref()
                                    .unwrap_or(&"".to_string())
                                    .clone(),
                                resource_ids: vec![resource.resource_id.unwrap_or(0) as i32],
                            }],
                            network_address: resource
                                .network_address
                                .as_ref()
                                .unwrap_or(&"".to_string())
                                .clone(),
                            resource_ids: vec![resource.resource_id.unwrap_or(0) as i32],
                        }],
                        resource_ids: vec![resource.resource_id.unwrap_or(0) as i32],
                    };

                    // Add the cluster to all_clusters
                    self.all_clusters.push(new_cluster);
                } else {
                    // if the cluster already exists, check if the host exists and add the host if it doesn't
                    let cluster = self
                        .all_clusters
                        .iter_mut()
                        .find(|cluster| cluster.name == cluster_name)
                        .unwrap();
                    if !cluster.hosts.iter().any(|host| {
                        host.name == resource.host.as_ref().unwrap_or(&"".to_string()).clone()
                    }) {
                        cluster.hosts.push(Host {
                            name: resource.host.as_ref().unwrap_or(&"".to_string()).clone(),
                            cpus: vec![Cpu {
                                name: resource
                                    .nodemodel
                                    .as_ref()
                                    .unwrap_or(&"".to_string())
                                    .clone(),
                                resources: vec![Resource {
                                    id: resource.resource_id.unwrap_or(0) as i32,
                                    state: match resource
                                        .state
                                        .as_ref()
                                        .unwrap_or(&"".to_string())
                                        .as_str()
                                    {
                                        "Dead" => super::resource::ResourceState::Dead,
                                        "Alive" => super::resource::ResourceState::Alive,
                                        "Absent" => super::resource::ResourceState::Absent,
                                        _ => super::resource::ResourceState::Unknown,
                                    },
                                    thread_count: resource.thread_count.unwrap_or(0) as i32,
                                }],
                                core_count: resource.core_count.unwrap_or(0) as i32,
                                cpufreq: resource
                                    .cpufreq
                                    .as_ref()
                                    .unwrap_or(&"".to_string())
                                    .parse::<f32>()
                                    .unwrap_or(0.0),
                                chassis: resource
                                    .chassis
                                    .as_ref()
                                    .unwrap_or(&"".to_string())
                                    .clone(),
                                resource_ids: vec![resource.resource_id.unwrap_or(0) as i32],
                            }],
                            network_address: resource
                                .network_address
                                .as_ref()
                                .unwrap_or(&"".to_string())
                                .clone(),
                            resource_ids: vec![resource.resource_id.unwrap_or(0) as i32],
                        });
                        // add the resource id to the cluster
                        cluster
                            .resource_ids
                            .push(resource.resource_id.unwrap_or(0) as i32);
                    } else {
                        // if the host already exists, check if the cpu exists and add the cpu if it doesn't
                        let host = cluster
                            .hosts
                            .iter_mut()
                            .find(|host| {
                                host.name
                                    == resource.host.as_ref().unwrap_or(&"".to_string()).clone()
                            })
                            .unwrap();
                        if !host.cpus.iter().any(|cpu| {
                            cpu.name
                                == resource
                                    .nodemodel
                                    .as_ref()
                                    .unwrap_or(&"".to_string())
                                    .clone()
                        }) {
                            host.cpus.push(Cpu {
                                name: resource
                                    .nodemodel
                                    .as_ref()
                                    .unwrap_or(&"".to_string())
                                    .clone(),
                                resources: vec![Resource {
                                    id: resource.resource_id.unwrap_or(0) as i32,
                                    state: match resource
                                        .state
                                        .as_ref()
                                        .unwrap_or(&"".to_string())
                                        .as_str()
                                    {
                                        "Dead" => super::resource::ResourceState::Dead,
                                        "Alive" => super::resource::ResourceState::Alive,
                                        "Absent" => super::resource::ResourceState::Absent,
                                        _ => super::resource::ResourceState::Unknown,
                                    },
                                    thread_count: resource.thread_count.unwrap_or(0) as i32,
                                }],
                                core_count: resource.core_count.unwrap_or(0) as i32,
                                cpufreq: resource
                                    .cpufreq
                                    .as_ref()
                                    .unwrap_or(&"".to_string())
                                    .parse::<f32>()
                                    .unwrap_or(0.0),
                                chassis: resource
                                    .chassis
                                    .as_ref()
                                    .unwrap_or(&"".to_string())
                                    .clone(),
                                resource_ids: vec![resource.resource_id.unwrap_or(0) as i32],
                            });

                            // add the resource id to the host and the cluster
                            host.resource_ids
                                .push(resource.resource_id.unwrap_or(0) as i32);
                            cluster
                                .resource_ids
                                .push(resource.resource_id.unwrap_or(0) as i32);
                        } else {
                            // if the cpu already exists, add the resource to the cpu
                            let cpu = host
                                .cpus
                                .iter_mut()
                                .find(|cpu| {
                                    cpu.name
                                        == resource
                                            .nodemodel
                                            .as_ref()
                                            .unwrap_or(&"".to_string())
                                            .clone()
                                })
                                .unwrap();
                            cpu.resources.push(Resource {
                                id: resource.resource_id.unwrap_or(0) as i32,
                                state: match resource
                                    .state
                                    .as_ref()
                                    .unwrap_or(&"".to_string())
                                    .as_str()
                                {
                                    "Dead" => super::resource::ResourceState::Dead,
                                    "Alive" => super::resource::ResourceState::Alive,
                                    "Absent" => super::resource::ResourceState::Absent,
                                    _ => super::resource::ResourceState::Unknown,
                                },
                                thread_count: resource.thread_count.unwrap_or(0) as i32,
                            });

                            // add the resource id to the cpu, the host and the cluster
                            cpu.resource_ids
                                .push(resource.resource_id.unwrap_or(0) as i32);
                            host.resource_ids
                                .push(resource.resource_id.unwrap_or(0) as i32);
                            cluster
                                .resource_ids
                                .push(resource.resource_id.unwrap_or(0) as i32);
                        }
                    }
                }
            }
        }
    }

    pub fn check_data_update(&mut self) {
        self.check_job_update();
        self.check_ressource_update();
    }

    //gather all unique owners (for completion in filters)
    pub fn get_unique_owners(&self) -> Vec<String> {
        let mut owners: Vec<String> = self.all_jobs.iter().map(|job| job.owner.clone()).collect();
        owners.sort();
        owners.dedup();
        owners
    }

    // Convert all_jobs to filtred_jobs applying some filters
    pub fn filter_jobs(&mut self) {
        self.filtered_jobs = self
            .all_jobs
            .iter()
            .filter(|job| {
                (self.filters.job_id_range.is_none() || {
                    let (start_id, end_id) = self.filters.job_id_range.unwrap();
                    job.id >= start_id && job.id <= end_id
                }) && (self
                    .filters
                    .owners
                    .as_ref()
                    .map_or(true, |owners| owners.contains(&job.owner)))
                    && (self
                        .filters
                        .states
                        .as_ref()
                        .map_or(true, |states| states.contains(&job.state)))
                    && (self
                        .filters
                        .scheduled_start_time
                        .map_or(true, |time| job.scheduled_start == time))
                    && (self
                        .filters
                        .wall_time
                        .map_or(true, |time| job.walltime == time))
            })
            .cloned() // Clone filtred jobs here
            .collect();
    }
}

impl Default for ApplicationContext {
    fn default() -> Self {
        let (jobs_sender, jobs_receiver) = channel();
        let (resources_sender, resources_receiver) = channel();

        let now: DateTime<Utc> = Utc::now();
        let mut context = Self {
            all_jobs: Vec::new(),
            all_clusters: Vec::new(),

            jobs_receiver: jobs_receiver,
            jobs_sender: jobs_sender,
            resources_receiver: resources_receiver,
            resources_sender: resources_sender,

            filtered_jobs: Vec::new(),
            filters: JobFilters::default(),
            start_date: Arc::new(Mutex::new(now - chrono::Duration::hours(1))),
            end_date: Arc::new(Mutex::new(now + chrono::Duration::hours(1))),
            view_type: ViewType::Authentification,
            is_loading: false,
            is_refreshing: Arc::new(Mutex::new(false)),
            refresh_rate: Arc::new(Mutex::new(30)),
            job_table: JobTable::default(), // Initialize job_table
        };
        context.update_periodically();
        context
    }
}
