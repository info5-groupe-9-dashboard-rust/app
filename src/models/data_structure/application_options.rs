use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum LanguageOption {
    English,
    Français,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ApplicationOptions {
    pub selected_language: LanguageOption,
    pub font_size: i32,
}

impl Default for ApplicationOptions {
    fn default() -> Self {
        ApplicationOptions {
            selected_language: LanguageOption::English,
            font_size: 14,
        }
    }
}