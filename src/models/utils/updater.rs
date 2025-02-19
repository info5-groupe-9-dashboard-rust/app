use chrono::{DateTime, Local};

use std::time::Duration;

use crate::models::data_structure::application_context::ApplicationContext;
#[cfg(target_arch = "wasm32")]
use crate::models::job::mock_jobs;

#[cfg(not(target_arch = "wasm32"))]
use crate::models::utils::parser::get_current_jobs_for_period;
#[cfg(not(target_arch = "wasm32"))]
use std::thread;

use super::parser::{get_jobs_from_json, get_resources_from_json};

impl ApplicationContext {
    pub fn update_refresh_rate(&mut self, new_rate: u64) {
        let mut rate = self.refresh_rate.lock().unwrap();
        *rate = new_rate;
        println!("New refresh rate: {:?}", self.refresh_rate);
    }

    pub fn update_start_date(&mut self, new_start: DateTime<Local>) {
        let mut start = self.start_date.lock().unwrap();
        *start = new_start;
    }

    pub fn update_end_date(&mut self, new_end: DateTime<Local>) {
        let mut end = self.end_date.lock().unwrap();
        *end = new_end;
    }

    pub fn get_start_date(&self) -> DateTime<Local> {
        *self.start_date.lock().unwrap()
    }

    pub fn get_end_date(&self) -> DateTime<Local> {
        *self.end_date.lock().unwrap()
    }

    pub fn instant_update(&mut self) {
        let is_refreshing = self.is_refreshing.clone();

        // if the app is already refreshing, return
        if *is_refreshing.lock().unwrap() {
            print!("Already refreshing");
            return;
        }

        // set refreshing to true
        *is_refreshing.lock().unwrap() = true;

        // get dates
        let start = *self.start_date.lock().unwrap();
        let end = *self.end_date.lock().unwrap();

        let jobs_sender = self.jobs_sender.clone();
        let resources_sender = self.resources_sender.clone();
        // Get the data in a different thread
        #[cfg(not(target_arch = "wasm32"))]
        {
            let is_refreshing_clone = is_refreshing.clone();
            thread::spawn(move || {
                let res = get_current_jobs_for_period(start, end);
                if res {
                    let jobs = get_jobs_from_json("./data/data.json");
                    let resources = get_resources_from_json("./data/data.json");

                    jobs_sender.send(jobs).unwrap_or_else(|e| {
                        println!("Error while sending jobs: {}", e);
                    });

                    resources_sender.send(resources).unwrap_or_else(|e| {
                        println!("Error while sending resources: {}", e);
                    });
                } else {
                    // LOG ERROR
                    print!("Error while fetching data");
                }
                // set refreshing to false
                *is_refreshing_clone.lock().unwrap() = false;
            });
        }

        #[cfg(target_arch = "wasm32")]
        {
            // LOG DEBUG
            // log::info!("instant_update: start_date: {:?}, end_date: {:?}", start, end);
            let jobs = mock_jobs();
            sender.send(jobs).unwrap();
            // set refreshing to false
            *is_refreshing.lock().unwrap() = false;
        }
    }

    // In a different thread, update the data every refresh_rate seconds
    pub fn update_periodically(&mut self) {
        let rate = *self.refresh_rate.lock().unwrap();
        let jobs_sender = self.jobs_sender.clone();
        let resources_sender = self.resources_sender.clone();
        let start = *self.start_date.lock().unwrap();
        let end = *self.end_date.lock().unwrap();
        let is_refreshing = self.is_refreshing.clone();

        // Get the data in a different thread
        #[cfg(not(target_arch = "wasm32"))]
        {
            thread::spawn(move || {
                loop {
                    // Check if already refreshing
                    if *is_refreshing.lock().unwrap() {
                        print!("Already refreshing");
                        thread::sleep(Duration::from_secs(rate));
                        continue;
                    }

                    // Set refreshing to true
                    *is_refreshing.lock().unwrap() = true;

                    let res = get_current_jobs_for_period(start, end);
                    if res {
                        let jobs = get_jobs_from_json("./data/data.json");
                        let resources = get_resources_from_json("./data/data.json");

                        jobs_sender.send(jobs).unwrap_or_else(|e| {
                            println!("Error while sending jobs: {}", e);
                        });

                        resources_sender.send(resources).unwrap_or_else(|e| {
                            println!("Error while sending resources: {}", e);
                        });
                    } else {
                        // LOG ERROR
                        print!("Error while fetching data");
                    }

                    // Set refreshing to false
                    *is_refreshing.lock().unwrap() = false;

                    thread::sleep(Duration::from_secs(rate));
                }
            });
        }

        #[cfg(target_arch = "wasm32")]
        {
            // LOG DEBUG
            // log::info!("update_periodically: start_date: {:?}, end_date: {:?}", start, end);
            let jobs = mock_jobs();
            sender.send(jobs).unwrap();
        }
    }

    pub fn update_period(&mut self, start_date: DateTime<Local>, end_date: DateTime<Local>) {
        self.update_start_date(start_date);
        self.update_end_date(end_date);
        self.is_loading = true;

        // Clone necessary value
        let sender = self.jobs_sender.clone();
        let start = start_date;
        let end = end_date;

        // Get the data in a different thread
        #[cfg(not(target_arch = "wasm32"))]
        {
            thread::spawn(move || {
                let res = get_current_jobs_for_period(start, end);
                if res {
                    let jobs = get_jobs_from_json("./data/data.json");
                    sender.send(jobs).unwrap();
                } else {
                    // LOG ERROR
                    print!("Error while fetching data");
                }
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
