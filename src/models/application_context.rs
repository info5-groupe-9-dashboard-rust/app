use chrono::{DateTime, Utc};
use crate::views::view::ViewType;
use super::{job::Job, parser::get_current_jobs_for_period};
// Ajouter dans application_context.rs
use std::sync::mpsc::{channel, Sender, Receiver}; 
use std::thread;

pub struct ApplicationContext {
    pub all_jobs: Vec<Job>, 
    pub filtred_jobs: Vec<Job>, 
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub view_type: ViewType,
    pub jobs_receiver: Receiver<Vec<Job>>, 
    pub jobs_sender: Sender<Vec<Job>>,     
    pub is_loading: bool                   
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
    }
}

impl Default for ApplicationContext {
    fn default() -> Self {
        let (sender, receiver) = channel();
        let now: DateTime<Utc> = Utc::now();
        let mut context = Self {
            all_jobs: Vec::new(),
            filtred_jobs: Vec::new(),
            start_date: Utc::now(),
            end_date: Utc::now(), 
            view_type: ViewType::Dashboard,
            jobs_receiver: receiver,
            jobs_sender: sender,
            is_loading: false
        };
        let start = now - chrono::Duration::hours(1);
        let end = now + chrono::Duration::hours(1);
        context.update_period(start, end);
        context
    }
}