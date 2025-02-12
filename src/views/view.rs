use eframe::egui;

use crate::models::data_structure::application_context::ApplicationContext;

pub trait View {
    fn render(&mut self, ui: &mut egui::Ui, app: &mut ApplicationContext);
}

pub enum ViewType {
    Dashboard,
    Gantt,
    Authentification,
}
