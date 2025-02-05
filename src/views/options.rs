use eframe::egui::{self, RichText};
use crate::models::application_context::ApplicationContext;

use super::view::View;
use eframe::egui::Grid;

#[derive(Debug, PartialEq, Eq)]
enum LanguageOption {
    English,
    Français,
}

#[derive(Debug, PartialEq, Eq)]
enum ThemeOption {
    Light,
    Dark,
}

pub struct Options {
    selected_language: LanguageOption,
    selected_theme: ThemeOption,
    font_size: i32,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            selected_language: LanguageOption::English,
            selected_theme: ThemeOption::Dark,
            font_size: 16,
        }
    }
}

impl View for Options {

    fn render(&mut self, ui: &mut egui::Ui, app: &mut ApplicationContext) {

        ui.heading(RichText::new(t!("app.options.title")).strong());
        ui.add_space(8.0);

        Grid::new("options_grid")
            .num_columns(2)
            .spacing([10.0, 8.0])
            .striped(true)
            .show(ui, |ui| {
            /* Theme options */
            ui.label(t!("app.options.theme.title"));
            egui::ComboBox::from_label(t!("app.options.theme.choose"))
                .selected_text(format!("{:?}", self.selected_theme))
                .show_ui(ui, |ui| {
                if ui.selectable_value(&mut self.selected_theme, ThemeOption::Dark, t!("app.options.theme.dark")).clicked() {
                    ui.ctx().set_visuals(egui::Visuals::dark());
                }
                if ui.selectable_value(&mut self.selected_theme, ThemeOption::Light, t!("app.options.theme.light")).clicked() {
                    ui.ctx().set_visuals(egui::Visuals::light());
                }
                });
            ui.end_row();

            /* Language options */
            ui.label(t!("app.options.language.title"));
            egui::ComboBox::from_label(t!("app.options.language.choose"))
                .selected_text(format!("{:?}", self.selected_language))
                .show_ui(ui, |ui| {
                if ui.selectable_value(&mut self.selected_language, LanguageOption::English, t!("app.options.language.en")).clicked() {
                    rust_i18n::set_locale("en");
                }
                if ui.selectable_value(&mut self.selected_language, LanguageOption::Français, t!("app.options.language.fr")).clicked() {
                    rust_i18n::set_locale("fr");
                }
                });
            ui.end_row();

            /* Font size options */
            ui.label(t!("app.options.font_size.title"));
            egui::ComboBox::from_label(t!("app.options.font_size.choose"))
                .selected_text(format!("{}", self.font_size))
                .show_ui(ui, |ui| {
                    for size in 10..=30 {
                        if ui.selectable_value(&mut self.font_size, size, size.to_string()).clicked() {
                            let mut fonts = egui::FontDefinitions::default();
                            let mut style = egui::Style::default();

                            // Appliquer la taille à tous les styles de texte importants
                            for text_style in [
                                egui::TextStyle::Body,
                                egui::TextStyle::Button,
                                egui::TextStyle::Heading,
                                egui::TextStyle::Monospace,
                                egui::TextStyle::Small,
                            ] {
                                style.text_styles.insert(
                                    text_style,
                                    egui::FontId::new(size as f32, egui::FontFamily::Proportional),
                                );
                            }

                            ui.ctx().set_style(style);
                            ui.ctx().set_fonts(fonts);
                        }
                    }
                });
            ui.end_row();
        });

        ui.add_space(10.0);

        if ui.button("Save").clicked() {
            // app.save_settings();
        }
    }
}