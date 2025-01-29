use eframe::egui;
use crate::models::job::Job;

pub fn render_dashboard(ui: &mut egui::Ui) {
    let jobs = Job::get_jobs_from_json("src/data/jobs.json");

    ui.heading("Tableau de bord");
    ui.add_space(8.0);

    // Statistics cards
    ui.horizontal(|ui| {
        stat_card(ui, "Nombre total de jobs", jobs.len());
        ui.add_space(8.0);
        stat_card(ui, "Jobs en cours", jobs.iter().filter(|j| j.state == "running").count());
        ui.add_space(8.0);
        stat_card(ui, "Jobs en attente", jobs.iter().filter(|j| j.state == "waiting").count());
    });

    ui.add_space(16.0);
    ui.heading("Liste des jobs");
    ui.add_space(8.0);

    // Table header with consistent spacing
    let table_headers = ["ID", "Propriétaire", "État", "File", "Temps alloué", "Commande"];
    ui.horizontal(|ui| {
        for header in table_headers {
            ui.add(egui::Label::new(egui::RichText::new(header).strong()));
            ui.add_space(20.0);
        }
    });

    ui.add_space(4.0);
    ui.separator();

    // Jobs list with improved layout
    egui::ScrollArea::vertical().show(ui, |ui| {
        for job in jobs {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                let color = match job.state.as_str() {
                    "running" => egui::Color32::from_rgb(76, 175, 80),
                    "waiting" => egui::Color32::from_rgb(255, 193, 7),
                    "completed" => egui::Color32::from_rgb(33, 150, 243),
                    "failed" => egui::Color32::from_rgb(244, 67, 54),
                    _ => egui::Color32::GRAY,
                };

                ui.label(format!("{}", job.id));
                ui.add_space(20.0);
                ui.label(&job.owner);
                ui.add_space(20.0);
                ui.colored_label(color, &job.state);
                ui.add_space(20.0);
                ui.label(&job.queue);
                ui.add_space(20.0);
                ui.label(format!("{}h", job.walltime));
                ui.add_space(20.0);
                ui.label(&job.command);

                if let Some(msg) = &job.message {
                    ui.add_space(20.0);
                    ui.colored_label(egui::Color32::RED, msg);
                }
            });
        }
    });
}

fn stat_card(ui: &mut egui::Ui, title: &str, value: usize) {
    ui.group(|ui| {
        ui.vertical(|ui| {
            ui.label(title);
            ui.heading(value.to_string());
        });
    });
}