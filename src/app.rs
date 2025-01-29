use eframe::egui;
use crate::views::{render_view, View};
use crate::views::menu::Menu;

pub struct App {
    pub current_view: View,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_view: View::Dashboard,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            Menu::render(ui, self);
        });

        egui::CentralPanel::default().show(ctx, |ui| render_view(ui, &self.current_view));
    }

}