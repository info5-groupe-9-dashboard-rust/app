// filepath: /home/rmiras/projects/rust-dashboard-app/src/views/components/metrics/metric_chart.rs
use eframe::egui;
use egui_plot::{Plot, Line};

#[derive(Clone)]
pub struct MetricChart {
    pub title: String,
    pub data: Vec<[f64; 2]>, // (x, y) pairs for the chart
}

impl MetricChart {
    pub fn new(title: &str) -> Self {
        MetricChart {
            title: title.to_string(),
            data: Vec::new(),
        }
    }

    pub fn add_data_point(&mut self, x: f64, y: f64) {
        self.data.push((x, y).into());
    }

    pub fn ui(&self, ui: &mut egui::Ui) {
        ui.label(&self.title);
        Plot::new(&self.title)
            .view_aspect(2.0)
            .show(ui, |plot_ui| {
                plot_ui.line(Line::new(self.data.clone()));
            });
    }
}