use crate::{
    models::data_structure::{
        application_context::ApplicationContext, application_options::ApplicationOptions,
    },
    views::view::View,
};
use eframe::egui;

use super::options::Options;

pub struct Menu {
    options_pane: Options,
}

impl Default for Menu {
    fn default() -> Self {
        let application_options = ApplicationOptions::default();

        let options_pane = if std::path::Path::new("options.json").exists() {
            Options::load_from_file("options.json")
        } else {
            Options::new(application_options.clone())
        };

        Menu { options_pane }
    }
}

impl View for Menu {
    fn render(&mut self, ui: &mut egui::Ui, app: &mut ApplicationContext) {
        self.options_pane
            .apply_options(ui.ctx(), &mut app.font_size);

        ui.horizontal(|ui| {
            // Menu File
            ui.menu_button(t!("app.menu.file"), |ui| {
                ui.set_max_width(200.0);
                ui.vertical(|ui| {
                    if app.user_connected.is_some() {
                        ui.label(t!(
                            "app.menu.connected_as",
                            user = app.user_connected.as_ref().unwrap()
                        ));
                        ui.separator();
                        if ui.button(t!("app.menu.logout")).clicked() {
                            app.logout();
                        }
                    } else {
                        if ui.button(t!("app.menu.login")).clicked() {
                            app.view_type = crate::views::view::ViewType::Authentification;
                        }
                    }

                    if ui.button(t!("app.menu.quit")).clicked() {
                        std::process::exit(0);
                    }
                });
            });

            // Menu Options
            if ui.button(t!("app.menu.options")).clicked() {
                self.options_pane.open();
            }

            // Show External Window
            self.options_pane.ui(ui, &mut app.font_size);
        });
    }
}
