use eframe::egui;
use crate::models::job::Job;
use super::view::View;

pub struct GanttChart {
    pub jobs: Vec<Job>,
    pub zoom: f32
}


impl GanttChart {
    pub fn new(jobs: Vec<Job>) -> Self {
        GanttChart {
            jobs : jobs,
            zoom: 1.0
        }
    }

    pub fn update_jobs(&mut self, jobs: Vec<Job>) {
        self.jobs = jobs;
    }
}


// Implement the View trait for GanttChart
impl View for GanttChart {
    
    fn render(&mut self, ui: &mut egui::Ui) {
        ui.heading("Diagramme de Gantt - Jobs OAR");

        let min_start = self.jobs.iter().map(|job| job.scheduled_start).min().unwrap_or(0);
        let max_end = self.jobs.iter().map(|job| job.scheduled_start + job.walltime).max().unwrap_or(0);
        let total_duration = (max_end - min_start) as f32;

        ui.horizontal(|ui| {
            if ui.button("Zoom +").clicked() {
                self.zoom *= 1.2;
            }
            if ui.button("Zoom -").clicked() {
                self.zoom /= 1.2;
            }
        });


        let available_width = ui.available_width();
        let scale = available_width / total_duration * self.zoom;

        egui::ScrollArea::horizontal().show(ui, |ui| {
            for job in &self.jobs {
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
