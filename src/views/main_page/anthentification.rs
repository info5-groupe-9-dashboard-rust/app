use crate::{models::data_structure::application_context::ApplicationContext, views::view::View};
use eframe::egui::{self, RichText};

pub struct Authentification {
    username: String,
    password: String,
    error_message: Option<String>,
}

impl Default for Authentification {
    fn default() -> Self {
        Authentification {
            username: String::new(),
            password: String::new(),
            error_message: None,
        }
    }
}

impl View for Authentification {
    fn render(&mut self, ui: &mut egui::Ui, app: &mut ApplicationContext) {
        // Center the form
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            
            // Title
            ui.heading(RichText::new("Authentification").size(24.0));
            ui.add_space(20.0);

            // Container of the form
            egui::Frame::none()
                .fill(ui.visuals().window_fill())
                .rounding(8.0)
                .inner_margin(20.0)
                .show(ui, |ui| {
                    ui.set_width(300.0);
                    
                    // username field
                    ui.label(RichText::new("Nom d'utilisateur"));
                    ui.add_space(4.0);
                    ui.text_edit_singleline(&mut self.username);
                    ui.add_space(12.0);

                    // password field
                    ui.label(RichText::new("Mot de passe"));
                    ui.add_space(4.0);
                    let password_edit = ui.add(egui::TextEdit::singleline(&mut self.password)
                        .password(true));
                    ui.add_space(20.0);

                    // Error message
                    if let Some(error) = &self.error_message {
                        ui.colored_label(egui::Color32::RED, error);
                        ui.add_space(8.0);
                    }

                    // Connect button
                    if ui.button("Se connecter").clicked() || password_edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        if self.username == "admin" && self.password == "admin" {
                            app.login(&self.username.clone());
                        } else {
                            self.error_message = Some("Identifiants incorrects".to_string());
                        }

                        self.password.clear();
                    }
                });
        });
    }
}