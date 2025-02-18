use crate::models::data_structure::job::JobState;
use crate::views::components::dashboard_components::job_table::JobTable;
use crate::views::components::dashboard_components::metric_box::MetricBox;
use crate::views::components::dashboard_components::metric_grid::MetricGrid;
use crate::{models::data_structure::application_context::ApplicationContext, views::view::View};
use eframe::egui::{self, RichText};
use strum::IntoEnumIterator;

pub struct Dashboard {
    job_table: JobTable,
    metric_grid: MetricGrid,
}

impl Default for Dashboard {
    fn default() -> Self {
        Dashboard {
            job_table: JobTable::default(),
            metric_grid: MetricGrid::default(),
        }
    }
}

impl View for Dashboard {
    fn render(&mut self, ui: &mut egui::Ui, app: &mut ApplicationContext) {

            egui::TopBottomPanel::top("title").show(ui.ctx(), |ui| {
                ui.heading(RichText::new(t!("app.dashboard.title")).strong().size(20.0));
            });

            egui::CentralPanel::default().show(ui.ctx(), |ui| {
                self.metric_grid.show(ui, |grid| {
                    
                    // Add total jobs metric
                    grid.add_metric(MetricBox::new(
                        t!("app.dashboard.total_jobs").to_string(),
                        app.filtered_jobs.len().to_string(),
                        egui::Color32::from_rgb(70, 100, 150),
                    ));
                    
                    
                   // N'ajouter que les métriques avec un compteur > 0
                    for state in JobState::iter() {
                        let count = app.filtered_jobs
                            .iter()
                            .filter(|j| j.state == state)
                            .count();

                        if count > 0 {
                            let translation_key = format!("app.job_state.{}", state.to_string().to_lowercase());
                            grid.add_metric(MetricBox::new(
                                t!(&translation_key).to_string(),
                                count.to_string(),
                                state.get_color().0,
                            ));
                        }
                    }
                });

                ui.add_space(10.0);
                ui.separator();

                // Draw the job table
                self.job_table.ui(ui, &app.filtered_jobs);
            });

    }
}
