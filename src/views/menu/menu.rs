use crate::{
    models::data_structure::{
        application_context::ApplicationContext, application_options::ApplicationOptions,
    },
    views::{
        components::time_selector::TimeSelector,
        view::{View, ViewType},
    },
};
use eframe::egui;

use super::{filtering::Filtering, options::Options};

pub struct Menu {
    time_selector: TimeSelector,
    filtering_pane: Filtering,
    options_pane: Options,
}

impl Default for Menu {
    fn default() -> Self {
        // Get current egui context

        let application_options = ApplicationOptions::default();

        let options_pane = if std::path::Path::new("options.json").exists() {
            Options::load_from_file("options.json")
        } else {
            Options::new(application_options.clone())
        };

        Menu {
            time_selector: TimeSelector::default(),
            filtering_pane: Filtering::default(),
            options_pane,
        }
    }
}

impl View for Menu {
    fn render(&mut self, ui: &mut egui::Ui, app: &mut ApplicationContext) {
        self.options_pane.apply_options(ui.ctx());

        ui.horizontal(|ui| {
            // Menu Fichier
            ui.menu_button(t!("app.menu.file"), |ui| {
                if ui.button("Quitter").clicked() {
                    std::process::exit(0);
                }
                if ui.button(t!("app.menu.logout")).clicked() {
                    // app.logout();
                }
            });

            // Menu Options
            if ui.button(t!("app.menu.options")).clicked() {
                self.options_pane.open();
            }

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

            // Menu Filters
            if ui.button(t!("app.menu.filters")).clicked() {
                self.filtering_pane.open();
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

            // Show External Window
            self.time_selector.ui(ui, app);
            self.filtering_pane.ui(ui, app);
            self.options_pane.ui(ui);
        });
    }
}
