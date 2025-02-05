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
            ui.menu_button(t!("app.menu.file"), |ui| {
                if ui.button("Quitter").clicked() {
                    std::process::exit(0);
                }
            });

            // Menu Vue
            ui.menu_button(t!("app.menu.view"), |ui| {
                if ui.button(t!("app.menu.dashboard")).clicked() {
                    app.view_type = ViewType::Dashboard;
                    ui.close_menu();
                }
                if ui.button(t!("app.menu.gantt")).clicked() {
                    app.view_type = ViewType::Gantt;
                    ui.close_menu();
                }
            });

            // Menu Language
            ui.menu_button(t!("app.menu.language.title"), |ui| {
                if ui.button(t!("app.menu.language.en")).clicked() {
                    rust_i18n::set_locale("en");
                    ui.close_menu();
                }
                if ui.button(t!("app.menu.language.fr")).clicked() {
                    rust_i18n::set_locale("fr");
                    ui.close_menu();
                }
            });

            self.time_selector.ui(ui, app);
        });
    }
}
