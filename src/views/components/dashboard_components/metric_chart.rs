use egui::{Response, RichText, Vec2, Widget};
use egui_plot::{Bar, BarChart, BoxPlot, Plot};
use std::collections::HashMap;

use crate::models::data_structure::job::Job;

#[allow(dead_code)]
pub enum ChartType {
    Bar(BarChart),
    Box(BoxPlot),
    // Add more chart types here
}

pub struct MetricChart {
    pub title: String,
    pub chart: ChartType,
    pub color: egui::Color32,
}

impl MetricChart {
    const MIN_WIDTH: f32 = 200.0;
    const MIN_HEIGHT: f32 = 150.0;

    pub fn new(title: &str, chart: ChartType) -> Self {
        MetricChart {
            title: title.to_string(),
            chart,
            color: egui::Color32::from_rgb(70, 100, 150),
        }
    }

    pub fn ui_with_chart(self, ui: &mut egui::Ui, size: Vec2) -> Response {
        egui::Frame::none()
            .fill(egui::Color32::from_gray(28))
            .rounding(6.0)
            .stroke(egui::Stroke::new(0.5, self.color))
            .show(ui, |ui| {
                ui.set_min_size(size);
                ui.vertical_centered(|ui| {
                    let title_size = (13.0 * size.x / Self::MIN_WIDTH).min(16.0);

                    ui.add_space(size.y * 0.1);
                    ui.label(
                        RichText::new(&self.title)
                            .color(egui::Color32::from_gray(160))
                            .size(title_size),
                    );
                    ui.add_space(size.y * 0.15);
                    Plot::new("metric_chart")
                        .view_aspect(2.0)
                        .show(ui, |plot_ui| match self.chart {
                            ChartType::Bar(bar_chart) => plot_ui.bar_chart(bar_chart),
                            ChartType::Box(boxplot_chart) => plot_ui.box_plot(boxplot_chart),
                        })
                        .response // Return the response from the Plot widget
                });
            })
            .response
    }
}

impl Widget for MetricChart {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        self.ui_with_chart(ui, Vec2::new(Self::MIN_WIDTH, Self::MIN_HEIGHT))
    }
}

pub fn create_jobstate_chart(jobs: Vec<Job>) -> MetricChart {
    let mut job_states = HashMap::new();
    for job in jobs {
        let count = job_states.entry(job.state).or_insert(0);
        *count += 1;
    }

    // Convert HashMap to Vec and sort by state label
    let mut state_counts: Vec<_> = job_states.into_iter().collect();
    state_counts.sort_by_key(|(state, _)| state.get_label());

    let bars: Vec<Bar> = state_counts
        .into_iter()
        .enumerate()
        .map(|(index, (state, count))| {
            Bar::new(index as f64, count as f64)
                .name(state.get_label())
                .fill(state.get_color().0)
        })
        .collect();

    let plot = BarChart::new(bars).name("Job States");
    MetricChart::new("Job State", ChartType::Bar(plot))
}
