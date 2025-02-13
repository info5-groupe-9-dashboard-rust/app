use super::{
    components::time_selector::TimeSelector,
    filtering::{self, Filtering},
    view::{View, ViewType},
};
use crate::models::application_context::ApplicationContext;
use eframe::egui;

pub struct Menu {
    time_selector: TimeSelector,
    filtering_pane: Filtering,
}

impl Default for Menu {
    fn default() -> Self {
        Menu {
            time_selector: TimeSelector::default(),
            filtering_pane: Filtering::default(),
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

            // Menu Filters
            if ui.button(t!("app.menu.filters")).clicked() {
                self.filtering_pane.open();
            }
            self.filtering_pane.ui(ui, app);

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
