use eframe::egui;

pub trait View {
    fn render(&mut self, ui: &mut egui::Ui);
}
