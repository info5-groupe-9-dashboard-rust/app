use crate::models::data_structure::job::JobState;
use crate::views::components::dashboard_components::job_table::JobTable;
use crate::views::components::dashboard_components::metric_box::MetricBox;
use crate::views::components::dashboard_components::metric_chart::create_jobstate_chart;
use crate::views::components::dashboard_components::metric_grid::MetricGrid;
use crate::{models::data_structure::application_context::ApplicationContext, views::view::View};
use eframe::egui::{self, RichText};
use strum::IntoEnumIterator;

pub struct Dashboard {
    job_table: JobTable,
    metric_grid: MetricGrid,
    show_chart: bool, // Add this field
}

impl Default for Dashboard {
    fn default() -> Self {
        Dashboard {
            job_table: JobTable::default(),
            metric_grid: MetricGrid::default(),
            show_chart: false, // Initialize the field
        }
    }
}

impl View for Dashboard {
    fn render(&mut self, ui: &mut egui::Ui, app: &mut ApplicationContext) {
        egui::CentralPanel::default().show(ui.ctx(), |ui| {
            ui.heading(RichText::new(t!("app.dashboard.title")).strong().size(20.0));

            // Add a button to toggle between the job chart and the job state metrics
            if ui
                .button(if self.show_chart {
                    "Show Metrics"
                } else {
                    "Show Chart"
                })
                .clicked()
            {
                self.show_chart = !self.show_chart;
            }

            self.metric_grid.show(ui, |grid| {
                if self.show_chart {
                    // Add the job state chart
                    let chart = create_jobstate_chart(app.filtered_jobs.clone());
                    grid.add_chart(chart);
                } else {
                    // Add total jobs metric
                    grid.add_metric(MetricBox::new(
                        t!("app.dashboard.total_jobs").to_string(),
                        app.filtered_jobs.len().to_string(),
                        egui::Color32::from_rgb(128, 128, 128),
                    ));

                    // Add the job state metrics
                    for state in JobState::iter() {
                        let count = app
                            .filtered_jobs
                            .iter()
                            .filter(|j| j.state == state)
                            .count();

                        if count > 0 {
                            let translation_key =
                                format!("app.job_state.{}", state.to_string().to_lowercase());
                            grid.add_metric(MetricBox::new(
                                t!(&translation_key).to_string(),
                                count.to_string(),
                                state.get_color().1,
                            ));
                        }
                    }
                    let start_time = app.get_start_date();
                    let end_time = app.get_end_date();

                    // Add time range metrics
                    grid.add_metric(MetricBox::new(
                        t!("app.dashboard.start_time").to_string(),
                        start_time.format("%Y-%m-%d %H:%M").to_string(),
                        egui::Color32::from_rgb(70, 130, 180),
                    ));

                    grid.add_metric(MetricBox::new(
                        t!("app.dashboard.end_time").to_string(),
                        end_time.format("%Y-%m-%d %H:%M").to_string(),
                        egui::Color32::from_rgb(70, 130, 180),
                    ));
                }
            });

            ui.add_space(10.0);
            ui.separator();

            // Draw the job table
            self.job_table.ui(ui, app);
        });
    }
}
