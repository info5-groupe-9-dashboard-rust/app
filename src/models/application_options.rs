use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum LanguageOption {
    English,
    Français,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ThemeOption {
    Light,
    Dark,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ApplicationOptions {
    pub selected_language: LanguageOption,
    pub selected_theme: ThemeOption,
    pub font_size: i32,
}

impl Default for ApplicationOptions {
    fn default() -> Self {
        ApplicationOptions {
            selected_language: LanguageOption::English,
            selected_theme: ThemeOption::Dark,
            font_size: 14,
        }
    }
}