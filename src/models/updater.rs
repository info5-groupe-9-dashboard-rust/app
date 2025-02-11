use super::application_context::ApplicationContext;
use chrono::Utc;
use std::sync::Arc;
use chrono::DateTime;

use std::time::Duration;

#[cfg(target_arch = "wasm32")]
use crate::models::job::mock_jobs;

#[cfg(not(target_arch = "wasm32"))]
use crate::models::parser::get_current_jobs_for_period;
#[cfg(not(target_arch = "wasm32"))]
use std::thread;

impl ApplicationContext {
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
        #[cfg(not(target_arch = "wasm32"))]
        {
            let start_date = Arc::clone(&self.start_date);
            let end_date = Arc::clone(&self.end_date);
            let refresh_rate = Arc::clone(&self.refresh_rate);
            let sender = self.jobs_sender.clone();
            thread::spawn(move || loop {
                let start = *start_date.lock().unwrap();
                let end = *end_date.lock().unwrap();
                let jobs = get_current_jobs_for_period(start, end);
                sender.send(jobs).unwrap();
                let rate = *refresh_rate.lock().unwrap();
                thread::sleep(Duration::from_secs(rate));
            });
        }
    }

    pub fn update_period(&mut self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) {
        self.update_start_date(start_date);
        self.update_end_date(end_date);
        self.is_loading = true;

        // Cloner les valeurs nécessaires
        let sender = self.jobs_sender.clone();
        let start = start_date;
        let end = end_date;

        // Lancer dans un thread séparé
        #[cfg(not(target_arch = "wasm32"))]
        {
            thread::spawn(move || {
                let jobs = get_current_jobs_for_period(start, end);
                sender.send(jobs).unwrap();
            });
        }

        #[cfg(target_arch = "wasm32")]
        {
            // LOG DEBUG
            // log::info!("update_period: start_date: {:?}, end_date: {:?}", start, end);
            let jobs = mock_jobs();
            sender.send(jobs).unwrap();
        }
    }
}