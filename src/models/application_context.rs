use chrono::{DateTime, Utc};
use crate::views::view::ViewType;
use super::{job::Job, parser::get_current_jobs_for_period};
// Ajouter dans application_context.rs
use std::sync::mpsc::{channel, Sender, Receiver}; 
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct ApplicationContext {
    pub all_jobs: Vec<Job>, 
    pub filtred_jobs: Vec<Job>, 
    pub start_date: Arc<Mutex<DateTime<Utc>>>,
    pub end_date: Arc<Mutex<DateTime<Utc>>>,
    pub view_type: ViewType,
    pub jobs_receiver: Receiver<Vec<Job>>, 
    pub jobs_sender: Sender<Vec<Job>>,     
    pub is_loading: bool,
    pub refresh_rate: Arc<Mutex<u64>>,
}

impl ApplicationContext {
    pub fn update_period(&mut self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) {
        self.update_start_date(start_date);
        self.update_end_date(end_date);
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

    pub fn update_refresh_rate(&mut self, new_rate: u64) {
        let mut rate = self.refresh_rate.lock().unwrap();
        *rate = new_rate;
    }

    pub fn update_start_date(&mut self, new_start: DateTime<Utc>) {
        let mut start = self.start_date.lock().unwrap();
        *start = new_start;
    }

    pub fn update_end_date(&mut self, new_end: DateTime<Utc>) {
        let mut end = self.end_date.lock().unwrap();
        *end = new_end;
    }

    pub fn get_start_date(&self) -> DateTime<Utc> {
        *self.start_date.lock().unwrap()
    }

    pub fn get_end_date(&self) -> DateTime<Utc> {
        *self.end_date.lock().unwrap()
    }

    pub fn update_periodically(&mut self) {
        let start_date = Arc::clone(&self.start_date);
        let end_date = Arc::clone(&self.end_date);
        let refresh_rate = Arc::clone(&self.refresh_rate);
        let sender = self.jobs_sender.clone();
        thread::spawn(move || {
            loop {
                let start = *start_date.lock().unwrap();
                let end = *end_date.lock().unwrap();
                let jobs = get_current_jobs_for_period(start, end);
                sender.send(jobs).unwrap();
                let rate = *refresh_rate.lock().unwrap();
                thread::sleep(Duration::from_secs(rate));
            }
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
        self.filtred_jobs = self.all_jobs.clone();
    }
}

impl Default for ApplicationContext {
    fn default() -> Self {
        let (sender, receiver) = channel();
        let now: DateTime<Utc> = Utc::now();
        let mut context = Self {
            all_jobs: Vec::new(),
            filtred_jobs: Vec::new(),
            start_date: Arc::new(Mutex::new(now - chrono::Duration::hours(1))),
            end_date: Arc::new(Mutex::new(now + chrono::Duration::hours(1))), 
            view_type: ViewType::Dashboard,
            jobs_receiver: receiver,
            jobs_sender: sender,
            is_loading: false,
            refresh_rate: Arc::new(Mutex::new(5)),
        };
        context.update_period(context.get_start_date(), context.get_end_date());
        context.update_periodically();
        context
    }
}