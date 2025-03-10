use crate::models::data_structure::application_options::{
    ApplicationOptions, LanguageOption, ThemeOption,
};
use eframe::egui::{self};
use std::time::{Duration, Instant};

use eframe::egui::Grid;

pub struct Options {
    open: bool,
    application_options: ApplicationOptions,
    save_status: Option<(String, Instant)>,
}

impl Options {
    pub fn open(&mut self) {
        self.open = true;
    }

    pub fn new(application_options: ApplicationOptions) -> Self {
        Options {
            application_options,
            save_status: None,
            open: false,
        }
    }

    // Save the current options to a file for persistence
    pub fn save_to_file(&mut self, file_path: &str) {
        match serde_json::to_string(&self.application_options)
            .and_then(|json| std::fs::write(file_path, json).map_err(serde_json::Error::io))
        {
            Ok(_) => {
                self.save_status =
                    Some((t!("app.options.save.success").to_string(), Instant::now()));
            }
            Err(_) => {
                self.save_status = Some((t!("app.options.save.error").to_string(), Instant::now()));
            }
        }
    }

    // Load the options from a file, or fallback to default if file read fails
    pub fn load_from_file(file_path: &str) -> Self {
        let application_options = match std::fs::read_to_string(file_path) {
            Ok(json) => {
                serde_json::from_str(&json).unwrap_or_else(|_| ApplicationOptions::default())
            }
            Err(_) => ApplicationOptions::default(), // Fallback to default if file read fails
        };

        Options {
            application_options,
            open: false,
            save_status: None, // No status message on initial load
        }
    }

    pub fn apply_options(&self, ctx: &egui::Context, app_font_size: &mut i32) {
        self.apply_theme(ctx);
        self.apply_language();
        self.apply_font_size(ctx, app_font_size);
    }

    fn apply_theme(&self, ctx: &egui::Context) {
        let new_visuals = match self.application_options.selected_theme {
            ThemeOption::Dark => egui::Visuals::dark(),
            ThemeOption::Light => egui::Visuals::light(),
        };

        if ctx.style().visuals.dark_mode != new_visuals.dark_mode {
            ctx.set_visuals(new_visuals);
        }
    }

    fn apply_language(&self) {
        rust_i18n::set_locale(match self.application_options.selected_language {
            LanguageOption::English => "en",
            LanguageOption::Français => "fr",
        });
    }

    fn apply_font_size(&self, ctx: &egui::Context, app_font_size: &mut i32) {
        let mut style = (*ctx.style()).clone();
        let font_size = self.application_options.font_size;

        for text_style in [
            egui::TextStyle::Body,
            egui::TextStyle::Button,
            egui::TextStyle::Heading,
            egui::TextStyle::Monospace,
            egui::TextStyle::Small,
        ] {
            style.text_styles.insert(
                text_style,
                egui::FontId::new(font_size as f32, egui::FontFamily::Proportional),
            );
        }

        ctx.set_style(style);
        *app_font_size = font_size;
    }
}

impl Options {
    pub fn ui(&mut self, ui: &mut egui::Ui, app_font_size: &mut i32) {
        let mut open = self.open; // Local copy to avoid borrowing issues

        egui::Window::new(t!("app.options.title"))
            .collapsible(true)
            .movable(true)
            .open(&mut open)
            .default_size([400.0, 500.0])
            .show(ui.ctx(), |ui| {
                Grid::new("options_grid")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .striped(true)
                    .show(ui, |ui| {
                        self.render_theme_selector(ui);
                        self.render_language_selector(ui);
                        self.render_font_size_selector(ui, app_font_size);
                    });

                ui.add_space(10.0);

                if ui.button(t!("app.options.save.title")).clicked() {
                    self.save_to_file("options.json");
                }

                // Show the save status message for a few seconds
                if let Some((message, timestamp)) = &self.save_status {
                    if timestamp.elapsed() < Duration::new(3, 0) {
                        ui.label(message);
                    } else {
                        self.save_status = None; // Clear the message after timeout
                    }
                }
            });
        self.open = open;
    }

    fn render_language_selector(&mut self, ui: &mut egui::Ui) {
        ui.label(t!("app.options.language.title"));
        egui::ComboBox::from_label(t!("app.options.language.choose"))
            .selected_text(format!("{:?}", self.application_options.selected_language))
            .show_ui(ui, |ui| {
                for (lang, label) in [
                    (LanguageOption::English, t!("app.options.language.en")),
                    (LanguageOption::Français, t!("app.options.language.fr")),
                ] {
                    if ui
                        .selectable_value(
                            &mut self.application_options.selected_language,
                            lang,
                            label,
                        )
                        .clicked()
                    {
                        self.apply_language();
                    }
                }
            });
        ui.end_row();
    }

    fn render_font_size_selector(&mut self, ui: &mut egui::Ui, app_font_size: &mut i32) {
        ui.label(t!("app.options.font_size.title"));
        egui::ComboBox::from_label(t!("app.options.font_size.choose"))
            .selected_text(format!("{}", self.application_options.font_size))
            .show_ui(ui, |ui| {
                for size in 10..=30 {
                    if ui
                        .selectable_value(
                            &mut self.application_options.font_size,
                            size,
                            size.to_string(),
                        )
                        .clicked()
                    {
                        self.apply_font_size(ui.ctx(), app_font_size);
                    }
                }
            });
        ui.end_row();
    }

    fn render_theme_selector(&mut self, ui: &mut egui::Ui) {
        ui.label(t!("app.options.theme.title"));
        egui::ComboBox::from_label(t!("app.options.theme.choose"))
            .selected_text(format!("{:?}", self.application_options.selected_theme))
            .show_ui(ui, |ui| {
                for (theme, label) in [
                    (ThemeOption::Dark, t!("app.options.theme.dark")),
                    (ThemeOption::Light, t!("app.options.theme.light")),
                ] {
                    if ui
                        .selectable_value(
                            &mut self.application_options.selected_theme,
                            theme,
                            label,
                        )
                        .clicked()
                    {
                        self.apply_theme(ui.ctx());
                    }
                }
            });
        ui.end_row();
    }
}
