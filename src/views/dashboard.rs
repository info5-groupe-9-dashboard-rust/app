use eframe::egui;
use crate::models::job::Job;
use super::view::View;

pub struct Dashboard {
    pub jobs: Vec<Job>
}

impl Dashboard {
    pub fn new(jobs: Vec<Job>) -> Self {
        Dashboard {
            jobs : jobs
        }
    }

    pub fn update_jobs(&mut self, jobs: Vec<Job>) {
        self.jobs = jobs;
    }
}

fn grafana_panel(ui: &mut egui::Ui, title: &str, value: usize, color: egui::Color32) {
    ui.group(|ui| {
        ui.style_mut().visuals.widgets.inactive.bg_fill = color;
        ui.vertical(|ui| {
            ui.label(title);
            ui.heading(value.to_string());
        });
    });
}

impl View for Dashboard {

    fn render(&mut self, ui: &mut egui::Ui) {

        fn stat_card(ui: &mut egui::Ui, title: &str, value: usize) {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label(title);
                    ui.heading(value.to_string());
                });
            });
        }

        ui.heading("Tableau de bord");
        ui.add_space(8.0);

        // Statistics cards
        ui.horizontal_wrapped(|ui| {
            grafana_panel(ui, "Total Jobs", self.jobs.len(), egui::Color32::from_rgb(40, 120, 215));
            ui.add_space(8.0);
            grafana_panel(ui, "Running", self.jobs.iter().filter(|j| j.state == "Running").count(), egui::Color32::from_rgb(235, 140, 50));
            ui.add_space(8.0);
            grafana_panel(ui, "Waiting", self.jobs.iter().filter(|j| j.state == "Waiting").count(), egui::Color32::from_rgb(200, 200, 50));
        });


        ui.add_space(16.0);
        ui.heading("Liste des jobs");
        ui.add_space(8.0);

        // Jobs table
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("ID");
                ui.label("Owner");
                ui.label("Etat");
                ui.label("Date de début");
                ui.label("Durée");
            });

            for job in &self.jobs {
                ui.horizontal_wrapped(|ui| {
                    ui.label(job.id.to_string());
                    ui.label(job.owner.clone());
                    ui.label(job.state.clone());
                    ui.label(job.scheduled_start.to_string());
                    ui.label(job.walltime.to_string());
                });
            }
        });
    }
}