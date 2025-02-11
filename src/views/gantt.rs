use crate::models::application_context::ApplicationContext;
use eframe::egui;

use super::view::View;

pub struct GanttChart {
    pub zoom: f32,
}

impl Default for GanttChart {
    fn default() -> Self {
        GanttChart { zoom: 1.0 }
    }
}

// Implement the View trait for GanttChart
impl View for GanttChart {
    fn render(&mut self, ui: &mut egui::Ui, app: &mut ApplicationContext) {
        ui.heading(t!("app.gantt.title"));

        let min_start = app
            .filtered_jobs
            .iter()
            .map(|job| job.scheduled_start)
            .min()
            .unwrap_or(0);
        let max_end = app
            .filtered_jobs
            .iter()
            .map(|job| job.scheduled_start + job.walltime)
            .max()
            .unwrap_or(0);
        let total_duration = (max_end - min_start) as f32;

        ui.horizontal(|ui| {
            if ui.button(t!("app.gantt.zoom_in")).clicked() {
                self.zoom *= 1.2;
            }
            if ui.button(t!("app.gantt.zoom_out")).clicked() {
                self.zoom /= 1.2;
            }
        });

        let available_width = ui.available_width();
        let scale = available_width / total_duration * self.zoom;

        egui::ScrollArea::horizontal().show(ui, |ui| {
            for job in &app.filtered_jobs {
                let start_pos = (job.scheduled_start - min_start) as f32 * scale;
                let duration_width = (job.walltime as f32) * scale;
                let color = egui::Color32::from_rgb((job.id * 37 % 255) as u8, 100, 200);

                ui.horizontal(|ui| {
                    ui.label(format!("Job {}", job.id));
                    ui.painter().rect_filled(
                        egui::Rect::from_min_size(
                            egui::pos2(start_pos, ui.cursor().top()),
                            egui::vec2(duration_width, 20.0),
                        ),
                        5.0,
                        color,
                    );
                });
            }
        });
    }
}
