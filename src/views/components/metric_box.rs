use eframe::egui;

pub struct MetricBox {
    title: String,
    value: usize,
    color: egui::Color32,
}

impl MetricBox {
    pub fn new(title: String, value: usize, color: egui::Color32) -> Self {
        Self { title, value, color }
    }

    pub fn ui(&self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.style_mut().visuals.widgets.inactive.bg_fill = self.color;
            ui.vertical(|ui| {
                ui.label(&self.title);
                ui.heading(self.value.to_string());
            });
        });
    }
}