use crate::{models::data_structure::application_context::ApplicationContext, views::{components::time_selector::TimeSelector, view::{View, ViewType}}};
use eframe::egui;

use super::filtering::Filtering;

#[allow(dead_code)]
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

            
            ui.label(t!("app.mode"));

            // Dashboard Button
            let is_dashboard_selected = matches!(app.view_type, ViewType::Dashboard);

            let dashboard_btn = egui::Button::new("📊 Dashboard").frame(is_dashboard_selected);
            if ui.add(dashboard_btn).clicked() {
                app.view_type = ViewType::Dashboard;
                ui.close_menu();
            }

            // Gantt Button
            let gantt_btn = egui::Button::new("📅 Gantt").frame(!is_dashboard_selected);
            if ui.add(gantt_btn).clicked() {
                app.view_type = ViewType::Gantt;
                ui.close_menu();
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {

                let refresh_btn = egui::Button::new("⟳");
                let refresh_btn_response = if *app.is_refreshing.lock().unwrap() {
                    ui.add_enabled(false, refresh_btn)
                } else {
                    ui.add(refresh_btn)
                };
                if refresh_btn_response.clicked() {
                    app.instant_update();
                }

                // Menu Refresh Rate
                ui.menu_button("🕓 ".to_string() + &t!("app.menu.refresh_rate.button"), |ui| {
                    ui.set_min_width(70.0); // Set the minimum width to 150.0

                    let refresh_rates = vec![
                        (30, t!("app.menu.refresh_rate.refresh_30")),
                        (60, t!("app.menu.refresh_rate.refresh_60")),
                        (300, t!("app.menu.refresh_rate.refresh_300")),
                    ];

                    for (rate, label) in refresh_rates {
                        let selected = *app.refresh_rate.lock().unwrap() == rate;
                        let display_label = if selected {
                            format!("{} ✔", label)
                        } else {
                            label.to_string()
                        };
                        if ui.selectable_label(selected, display_label).clicked() {
                            app.update_refresh_rate(rate);
                            ui.close_menu();
                        }
                    }
                });

                // Filters
                let filters_btn = egui::Button::new("🔎 ".to_string() + &t!("app.menu.filters")).frame(true);
                if ui.add(filters_btn).clicked() {
                    self.filtering_pane.open();
                }


            });

            // Show External Window
            //self.time_selector.ui(ui, app);
            self.filtering_pane.ui(ui, app);

        });
    }
}
