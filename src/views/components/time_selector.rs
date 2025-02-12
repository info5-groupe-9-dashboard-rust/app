use eframe::egui;
use chrono::{NaiveDate, NaiveTime, NaiveDateTime, Utc, TimeZone, Datelike, Duration};
use crate::models::data_structure::application_context::ApplicationContext;

pub struct TimeSelector {
    // État du sélecteur de période
    date_selector_open: bool,
    temp_start_date: String,
    temp_start_time: String,
    temp_end_date: String,
    temp_end_time: String,
    error: Option<String>,
}

impl Default for TimeSelector {
    fn default() -> Self {
        // On pré-remplit avec la date et l'heure actuelle par défaut
        let now = Utc::now();
        TimeSelector {
            date_selector_open: false,
            temp_start_date: now.date_naive().to_string(),
            temp_start_time: "00:00".to_owned(),
            temp_end_date: now.date_naive().to_string(),
            temp_end_time: "23:59".to_owned(),
            error: None,
        }
    }
}

impl TimeSelector {
    pub fn ui(&mut self, ui: &mut egui::Ui, app: &mut ApplicationContext) {
        // Bouton pour ouvrir le sélecteur de période
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(t!("app.time_selector.button")).clicked() {
                // Ouvrir la fenêtre modale en pré-remplissant les champs avec les valeurs actuelles
                self.date_selector_open = true;
                self.temp_start_date = app.get_start_date().date_naive().to_string();
                self.temp_start_time = app.get_start_date().format("%H:%M").to_string();
                self.temp_end_date = app.get_end_date().date_naive().to_string();
                self.temp_end_time = app.get_end_date().format("%H:%M").to_string();
                self.error = None;
            }
        });

        // Fenêtre modale pour la sélection de période
        if self.date_selector_open {
            egui::Window::new(t!("app.time_selector.modal.title"))
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ui.ctx(), |ui| {
                    ui.horizontal(|ui| {
                        // Boutons de presets
                        if ui.button(t!("app.time_selector.modal.today")).clicked() {
                            let today = Utc::now().date_naive();
                            self.temp_start_date = today.to_string();
                            self.temp_end_date = today.to_string();
                            self.temp_start_time = "00:00".to_owned();
                            self.temp_end_time = "23:59".to_owned();
                            self.error = None;
                        }
                        if ui.button(t!("app.time_selector.modal.week")).clicked() {
                            let today = Utc::now().date_naive();
                            // Déterminer le lundi et le dimanche de la semaine courante
                            let weekday = today.weekday().number_from_monday() as i64;
                            let monday = today - Duration::days(weekday - 1);
                            let sunday = monday + Duration::days(6);
                            self.temp_start_date = monday.to_string();
                            self.temp_end_date = sunday.to_string();
                            self.temp_start_time = "00:00".to_owned();
                            self.temp_end_time = "23:59".to_owned();
                            self.error = None;
                        }
                        if ui.button(t!("app.time_selector.modal.month")).clicked() {
                            let today = Utc::now().date_naive();
                            let start_of_month = NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap();
                            // Calcul du dernier jour du mois : on ajoute un mois puis on soustrait un jour
                            let end_of_month = if today.month() == 12 {
                                NaiveDate::from_ymd_opt(today.year() + 1, 1, 1).unwrap() - Duration::days(1)
                            } else {
                                NaiveDate::from_ymd_opt(today.year(), today.month() + 1, 1).unwrap() - Duration::days(1)
                            };
                            self.temp_start_date = start_of_month.to_string();
                            self.temp_end_date = end_of_month.to_string();
                            self.temp_start_time = "00:00".to_owned();
                            self.temp_end_time = "23:59".to_owned();
                            self.error = None;
                        }
                    });

                    ui.separator();

                    // Saisie manuelle des dates et heures
                    ui.label(t!("app.time_selector.modal.start_date") + " (YYYY-MM-DD):");
                    ui.text_edit_singleline(&mut self.temp_start_date);

                    ui.label(t!("app.time_selector.modal.start_time") + "(HH:MM):");
                    ui.text_edit_singleline(&mut self.temp_start_time);

                    ui.separator();

                    ui.label(t!("app.time_selector.modal.end_date") + " (YYYY-MM-DD):");
                    ui.text_edit_singleline(&mut self.temp_end_date);

                    ui.label(t!("app.time_selector.modal.end_time") + "(HH:MM):");
                    ui.text_edit_singleline(&mut self.temp_end_time);

                    // Affichage d'un message d'erreur si nécessaire
                    if let Some(ref err) = self.error {
                        ui.colored_label(egui::Color32::RED, err);
                    }

                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button(t!("app.time_selector.modal.cancel")).clicked() {
                            self.date_selector_open = false;
                        }

                        if ui.button(t!("app.time_selector.modal.validate")).clicked() {
                            // Validation des dates et heures
                            let start_date_res = NaiveDate::parse_from_str(&self.temp_start_date, "%Y-%m-%d");
                            let start_time_res = NaiveTime::parse_from_str(&self.temp_start_time, "%H:%M");
                            let end_date_res = NaiveDate::parse_from_str(&self.temp_end_date, "%Y-%m-%d");
                            let end_time_res = NaiveTime::parse_from_str(&self.temp_end_time, "%H:%M");

                            if let (Ok(start_date), Ok(start_time), Ok(end_date), Ok(end_time)) =
                                (start_date_res, start_time_res, end_date_res, end_time_res)
                            {
                                let start_datetime = NaiveDateTime::new(start_date, start_time);
                                let end_datetime = NaiveDateTime::new(end_date, end_time);

                                if start_datetime <= end_datetime {
                                    // Mise à jour des dates et heures dans l'ApplicationContext
                                    // app.start_date = Utc.from_utc_datetime(&start_datetime);
                                    // app.end_date = Utc.from_utc_datetime(&end_datetime);
                                    app.update_period(Utc.from_utc_datetime(&start_datetime), Utc.from_utc_datetime(&end_datetime));
                                    println!("Période mise à jour: {} - {}", app.get_start_date(), app.get_end_date());
                                    self.date_selector_open = false;
                                } else {
                                    self.error = Some(t!("app.time_selector.errors.end_before_start").to_string());
                                }
                            } else {
                                self.error = Some(t!("app.time_selector.errors.invalid_format").to_string());
                            }
                        }
                    });
                });
        }
    }
}