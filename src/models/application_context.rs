use super::filters::JobFilters;
use super::job::Job;
use crate::views::view::ViewType;
use chrono::{DateTime, Utc};
// Ajouter dans application_context.rs
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

#[cfg(target_arch = "wasm32")]
use crate::models::job::mock_jobs;


pub struct ApplicationContext {
    pub all_jobs: Vec<Job>,
    pub filtered_jobs: Vec<Job>,
    pub start_date: Arc<Mutex<DateTime<Utc>>>,
    pub end_date: Arc<Mutex<DateTime<Utc>>>,
    pub view_type: ViewType,
    pub jobs_receiver: Receiver<Vec<Job>>,
    pub jobs_sender: Sender<Vec<Job>>,
    pub is_loading: bool,
    pub refresh_rate: Arc<Mutex<u64>>,
    pub filters: JobFilters,
}

impl ApplicationContext {
    pub fn check_jobs_update(&mut self) {
        // Vérifier si de nouvelles données sont disponibles
        if let Ok(new_jobs) = self.jobs_receiver.try_recv() {
            self.all_jobs = new_jobs;
            self.is_loading = false;
        }
        self.filter_jobs();
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
                // Vos conditions de filtrage ici
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
            .cloned() // On clone ici les jobs filtrés
            .collect();
    }
}

impl Default for ApplicationContext {
    fn default() -> Self {
        let (sender, receiver) = channel();
        let now: DateTime<Utc> = Utc::now();
        let mut context = Self {
            all_jobs: Vec::new(),
            filtered_jobs: Vec::new(),
            filters: JobFilters::default(),
            start_date: Arc::new(Mutex::new(now - chrono::Duration::hours(1))),
            end_date: Arc::new(Mutex::new(now + chrono::Duration::hours(1))),
            view_type: ViewType::Dashboard,
            jobs_receiver: receiver,
            jobs_sender: sender,
            is_loading: false,
            refresh_rate: Arc::new(Mutex::new(30)),
        };
        context.update_periodically();
        context
    }
}
