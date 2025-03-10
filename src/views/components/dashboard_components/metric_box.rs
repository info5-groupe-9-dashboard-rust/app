use eframe::egui::{self, Color32, RichText, Vec2, Widget};
use egui::Response;

/**
 * Struct for the metric box
 */
#[derive(Clone)]
pub struct MetricBox {
    title: String, // Title of the metric box
    value: String, // Value of the metric box
    color: Color32, // Color of the metric box
}

/**
 * Implementation of the metric box
 */
impl MetricBox {
    pub const MIN_WIDTH: f32 = 180.0; // Minimum width of the metric box
    pub const MIN_HEIGHT: f32 = 90.0; // Minimum height of the metric box

    /**
     * Creates a new metric box
     */
    pub fn new(title: String, value: String, color: Color32) -> Self {
        MetricBox {
            title,
            value,
            color,
        }
    }

    /**
     * Shows the metric box with a specific size
     */
    pub fn ui_sized(self, ui: &mut egui::Ui, size: Vec2) -> Response {
        egui::Frame::none()
            .fill(if ui.ctx().style().visuals.dark_mode {
                egui::Color32::from_gray(28)
            } else {
                egui::Color32::from_gray(255)
            })
            .rounding(6.0)
            .stroke(egui::Stroke::new(0.7, self.color))
            .show(ui, |ui| {
                ui.set_min_size(size);
                ui.vertical_centered(|ui| {
                    let title_size = (13.0 * size.x / Self::MIN_WIDTH).min(16.0);
                    let value_size = (24.0 * size.x / Self::MIN_WIDTH).min(32.0);

                    ui.add_space(size.y * 0.1);
                    ui.label(
                        RichText::new(&self.title)
                            .color(egui::Color32::from_gray(160))
                            .size(title_size),
                    );
                    ui.add_space(size.y * 0.15);
                    ui.heading(
                        RichText::new(&self.value)
                            .color(self.color.gamma_multiply(0.8))
                            .size(value_size)
                            .strong(),
                    );
                });
            })
            .response
    }
}

/**
 * Implementation of the widget for the metric box
 */
impl Widget for MetricBox {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        self.ui_sized(ui, Vec2::new(Self::MIN_WIDTH, Self::MIN_HEIGHT))
    }
}
