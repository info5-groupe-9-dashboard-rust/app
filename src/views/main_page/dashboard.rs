use crate::{models::data_structure::application_context::ApplicationContext, views::view::View};
use crate::models::data_structure::job::JobState;
use crate::views::components::job_table::JobTable;
use crate::views::components::metric_box::MetricBox;
use eframe::egui::{self, RichText};

pub struct Dashboard {
    job_table: JobTable,
}

impl Default for Dashboard {
    fn default() -> Self {
        Dashboard {
            job_table: JobTable::default(),
        }
    }
}

impl View for Dashboard {
    fn render(&mut self, ui: &mut egui::Ui, app: &mut ApplicationContext) {
        /* Top panel : metrics */
        egui::TopBottomPanel::top("metrics").show(ui.ctx(), |ui| {
            ui.add_space(10.0);
            ui.heading(RichText::new(t!("app.dashboard.title")).strong());
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                let total_jobs = MetricBox::new(
                    t!("app.dashboard.metrics.total_jobs").to_string(),
                    app.filtered_jobs.len(),
                    egui::Color32::from_rgb(40, 120, 215),
                );
                total_jobs.ui(ui);
                ui.add_space(8.0);

                let running_jobs = MetricBox::new(
                    t!("app.dashboard.metrics.running").to_string(),
                    app.filtered_jobs
                        .iter()
                        .filter(|j| j.state == JobState::Running)
                        .count(),
                    egui::Color32::from_rgb(235, 140, 50),
                );
                running_jobs.ui(ui);
                ui.add_space(8.0);

                let waiting_jobs = MetricBox::new(
                    t!("app.dashboard.metrics.waiting").to_string(),
                    app.filtered_jobs
                        .iter()
                        .filter(|j| j.state == JobState::Waiting)
                        .count(),
                    egui::Color32::from_rgb(200, 200, 50),
                );
                waiting_jobs.ui(ui);
                ui.add_space(8.0);
            });
            ui.add_space(10.0);
        });

        /* Main panel : job table */
        self.job_table.ui(ui, &app.filtered_jobs);
    }
}
