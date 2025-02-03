use eframe::egui;
use crate::models::application_context::ApplicationContext;

use super::view::View;

pub struct Dashboard;

impl Default for Dashboard {
    fn default() -> Self {
        Dashboard {}
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

    fn render(&mut self, ui: &mut egui::Ui, app: &mut ApplicationContext) {

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
            grafana_panel(ui, "Total Jobs", app.jobs.len(), egui::Color32::from_rgb(40, 120, 215));
            ui.add_space(8.0);
            grafana_panel(ui, "Running", app.jobs.iter().filter(|j| j.state == "Running").count(), egui::Color32::from_rgb(235, 140, 50));
            ui.add_space(8.0);
            grafana_panel(ui, "Waiting", app.jobs.iter().filter(|j| j.state == "Waiting").count(), egui::Color32::from_rgb(200, 200, 50));
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

            for job in &app.jobs {
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