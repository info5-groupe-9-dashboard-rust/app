pub const GRID_SPACING_10: i64 = 10;
pub const GRID_SPACING_30: i64 = 30;
pub const GRID_SPACING_60: i64 = 60;

pub struct GridSpacing {
    pub value: i64,
}

impl Default for GridSpacing {
    fn default() -> Self {
        Self {
            value: GRID_SPACING_30,
        }
    }
}

impl GridSpacing {

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Grid spacing:");
            ui.horizontal(|ui|{
                ui.radio_value(&mut self.value, GRID_SPACING_10, "10 min");
                ui.radio_value(&mut self.value, GRID_SPACING_30, "30 min");
                ui.radio_value(&mut self.value, GRID_SPACING_60, "60 min");
            });
        });
    }
}