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

            // Menu Options
            if ui.button(t!("app.menu.options")).clicked() {
                app.view_type = ViewType::Options;
            }

            // Menu Refresh Rate
            ui.menu_button(t!("app.menu.refresh_rate.button"), |ui| {
                if ui.button(t!("app.menu.refresh_rate.refresh_30")).clicked() {
                    app.update_refresh_rate(30);
                    ui.close_menu();
                }
                if ui.button(t!("app.menu.refresh_rate.refresh_60")).clicked() {
                    app.update_refresh_rate(60);
                    ui.close_menu();
                }
                if ui.button(t!("app.menu.refresh_rate.refresh_300")).clicked() {
                    app.update_refresh_rate(300);
                    ui.close_menu();
                }
            });


            self.time_selector.ui(ui, app);
        });
    }
}
