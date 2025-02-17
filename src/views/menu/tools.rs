use crate::{models::data_structure::{application_context::ApplicationContext, application_options::ApplicationOptions}, views::{components::time_selector::TimeSelector, view::{View, ViewType}}};
use eframe::egui;
use egui::{include_image, Image, ImageSource, vec2 as size2};

use super::{filtering::Filtering};

pub struct Tools {
    time_selector: TimeSelector,
    filtering_pane: Filtering
}

impl Default for Tools {
    fn default() -> Self {

        Tools {
            time_selector: TimeSelector::default(),
            filtering_pane: Filtering::default()
        }
    }
}

impl View for Tools {
    fn render(&mut self, ui: &mut egui::Ui, app: &mut ApplicationContext) {

        ui.horizontal(|ui| {
            ui.set_height(25.0); // Set the height to 50.0

            // Dashboard Button
            let dashboard_btn = egui::Button::new("📊 Dashboard").frame(true);
            if ui.add(dashboard_btn).clicked() {
                app.view_type = ViewType::Dashboard;
                ui.close_menu();
            }

            // Gantt Button
            let gantt_btn = egui::Button::new("📅 Gantt").frame(true);
            if ui.add(gantt_btn).clicked() {
                app.view_type = ViewType::Gantt;
                ui.close_menu();
            }

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

        });
    }
}
