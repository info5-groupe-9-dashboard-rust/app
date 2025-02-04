use eframe::egui::{self, RichText};
use crate::models::application_context::ApplicationContext;
use crate::views::components::metric_box::MetricBox;
use crate::views::components::job_table::JobTable;

use super::view::View;

pub struct Dashboard;

impl Default for Dashboard {
    fn default() -> Self {
        Dashboard {}
    }
}

impl View for Dashboard {

    fn render(&mut self, ui: &mut egui::Ui, app: &mut ApplicationContext) {

        /* Top panel : metrics */
        egui::TopBottomPanel::top("metrics").show(ui.ctx(), |ui| {
            ui.add_space(10.0);
            ui.heading(RichText::new("Tableau de bord").strong());
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {

                let total_jobs = MetricBox::new(
                    "Total Jobs".to_string(),
                    app.jobs.len(),
                    egui::Color32::from_rgb(40, 120, 215),
                );
                total_jobs.ui(ui);
                ui.add_space(8.0);

                let running_jobs = MetricBox::new(
                    "Running".to_string(),
                    app.jobs.iter().filter(|j| j.state == "Running").count(),
                    egui::Color32::from_rgb(235, 140, 50),
                );
                running_jobs.ui(ui);
                ui.add_space(8.0);

                let waiting_jobs = MetricBox::new(
                    "Waiting".to_string(),
                    app.jobs.iter().filter(|j| j.state == "Waiting").count(),
                    egui::Color32::from_rgb(200, 200, 50),
                );
                waiting_jobs.ui(ui);
                ui.add_space(8.0);
            });
            ui.add_space(10.0);
        });

        /* Central panel : job list */
        egui::CentralPanel::default().show(ui.ctx(), |ui| {
            ui.add_space(10.0);
            ui.heading(RichText::new("Liste des jobs").strong());
            ui.add_space(8.0);
            let job_table = JobTable::new(&app.jobs);
            job_table.ui(ui);
            ui.add_space(10.0);
        });

    }
}