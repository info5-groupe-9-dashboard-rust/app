use chrono::{DateTime, Utc};
use crate::{models::parser::get_current_jobs, views::view::ViewType};
use super::job::Job;

impl Default for ApplicationContext {
    fn default() -> Self {
        ApplicationContext {
            jobs: get_current_jobs(),
            start_date: Utc::now(),
            end_date: Utc::now(),
            view_type: ViewType::Dashboard
        }
    }
}

pub struct ApplicationContext {
    pub jobs: Vec<Job>,
    pub start_date : DateTime<Utc>,
    pub end_date : DateTime<Utc>,
    pub view_type: ViewType
}