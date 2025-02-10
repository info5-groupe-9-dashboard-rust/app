use super::{filters::JobFilters, job::Job, parser::get_current_jobs_for_period};
use crate::views::view::ViewType;
use chrono::{DateTime, Utc};
// Ajouter dans application_context.rs
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub struct ApplicationContext {
    pub all_jobs: Vec<Job>,
    pub filtered_jobs: Vec<Job>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub view_type: ViewType,
    pub jobs_receiver: Receiver<Vec<Job>>,
    pub jobs_sender: Sender<Vec<Job>>,
    pub is_loading: bool,
    pub filters: JobFilters,
}

impl ApplicationContext {
    pub fn update_period(&mut self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) {
        self.start_date = start_date;
        self.end_date = end_date;
        self.is_loading = true;

        // Cloner les valeurs nécessaires
        let sender = self.jobs_sender.clone();
        let start = start_date;
        let end = end_date;

        // Lancer dans un thread séparé
        thread::spawn(move || {
            let jobs = get_current_jobs_for_period(start, end);
            sender.send(jobs).unwrap();
        });
    }

    pub fn check_jobs_update(&mut self) {
        // Vérifier si de nouvelles données sont disponibles
        if let Ok(new_jobs) = self.jobs_receiver.try_recv() {
            self.all_jobs = new_jobs;
            self.is_loading = false;
        }
        self.filter_jobs();
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
            start_date: Utc::now(),
            end_date: Utc::now(),
            view_type: ViewType::Dashboard,
            jobs_receiver: receiver,
            jobs_sender: sender,
            is_loading: false,
            filters: JobFilters::default(),
        };
        let start = now - chrono::Duration::hours(1);
        let end = now + chrono::Duration::hours(1);
        context.update_period(start, end);
        context
    }
}
