use eframe::egui;
use crate::models::application_context::ApplicationContext;
use super::{components::time_selector::TimeSelector, view::{View, ViewType}};

pub struct Menu {
    time_selector: TimeSelector,
}

impl Default for Menu {
    fn default() -> Self {
        Menu {
            time_selector: TimeSelector::default(),
        }
    }
}

impl View for Menu {
    fn render(&mut self, ui: &mut egui::Ui, app: &mut ApplicationContext) {
        ui.horizontal(|ui| {
            // Menu Fichier
            ui.menu_button("Fichier", |ui| {
                if ui.button("Quitter").clicked() {
                    std::process::exit(0);
                }
            });

            // Menu Vue
            ui.menu_button("Vue", |ui| {
                if ui.button("Tableau de bord").clicked() {
                    app.view_type = ViewType::Dashboard;
                    ui.close_menu();
                }
                if ui.button("Diagramme de Gantt").clicked() {
                    app.view_type = ViewType::Gantt;
                    ui.close_menu();
                }
            });
            self.time_selector.ui(ui, app);
        });
    }
}
