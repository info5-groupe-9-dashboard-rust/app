use eframe::egui::{self, Color32, RichText, Widget, Vec2};
use egui::Response;

#[derive(Clone)]
pub struct MetricBox {
    title: String,
    value: String,
    color: Color32,
}

impl MetricBox {
    pub const MIN_WIDTH: f32 = 180.0;
    pub const MIN_HEIGHT: f32 = 90.0;

    pub fn new(title: String, value: String, color: Color32) -> Self {
        MetricBox { title, value, color }
    }
}

impl Widget for MetricBox {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        let available_width = ui.available_width();
        let padding = 12.0;
        
        // Calcul de la taille adaptative
        let width = (available_width).max(Self::MIN_WIDTH);
        let height = Self::MIN_HEIGHT * (width / Self::MIN_WIDTH).sqrt();
        let desired_size = Vec2::new(width, height);

        let response = egui::Frame::none()
            .fill(egui::Color32::from_gray(28))
            .rounding(6.0)
            .stroke(egui::Stroke::new(0.5, self.color))
            .inner_margin(padding)
            .show(ui, |ui| {
                ui.set_min_size(desired_size);
                ui.vertical_centered(|ui| {
                    // Adapter la taille du texte en fonction de la largeur
                    let title_size = (13.0 * width / Self::MIN_WIDTH).min(16.0);
                    let value_size = (24.0 * width / Self::MIN_WIDTH).min(32.0);
                    
                    ui.add_space(height * 0.1);
                    ui.label(RichText::new(&self.title)
                        .color(egui::Color32::from_gray(160))
                        .size(title_size));
                    ui.add_space(height * 0.15);
                    ui.heading(RichText::new(&self.value)
                        .color(self.color.gamma_multiply(0.8))
                        .size(value_size)
                        .strong());
                });
            })
            .response;
        response
    }
}