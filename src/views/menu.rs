use crate::app::App;
use eframe::egui;

pub struct Menu;

impl Menu {
    pub fn render(ui: &mut egui::Ui, app: &mut App) {
        ui.horizontal(|ui| {
            ui.menu_button("Fichier", |ui| {
                if ui.button("Quitter").clicked() {
                    std::process::exit(0);
                }
            });
            ui.menu_button("Vue", |ui| {
                if ui.button("Tableau de bord").clicked() {
                    app.switch_to_dashboard();
                    ui.close_menu();
                }
                if ui.button("Diagramme de Gantt").clicked() {
                    app.switch_to_gantt();
                    ui.close_menu();
                }
            });
        });
    }
}