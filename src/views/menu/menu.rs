use crate::{
    models::data_structure::{
        application_context::ApplicationContext, application_options::ApplicationOptions,
    },
    views::view::View,
};
use eframe::egui;

use super::options::Options;

pub struct Menu {
    options_pane: Options
}

impl Default for Menu {
    fn default() -> Self {

        let application_options = ApplicationOptions::default();

        let options_pane = if std::path::Path::new("options.json").exists() {
            Options::load_from_file("options.json")
        } else {
            Options::new(application_options.clone())
        };

        Menu {
            options_pane,
        }
    }
}

impl View for Menu {
    fn render(&mut self, ui: &mut egui::Ui, _app: &mut ApplicationContext) {
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

            // Show External Window
            self.options_pane.ui(ui);
        });
    }
}
