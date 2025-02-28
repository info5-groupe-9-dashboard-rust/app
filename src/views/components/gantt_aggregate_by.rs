#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AggregateByLevel1Enum {
    Owner,
    Cluster,
    Host,
}

#[derive(PartialEq, Clone, Copy)]
pub enum AggregateByLevel2Enum {
    Owner,
    Host,
    None,
}

pub struct AggregateBy {
    pub level_1: AggregateByLevel1Enum,
    pub level_2: AggregateByLevel2Enum,
}

impl Default for AggregateBy {
    fn default() -> Self {
        Self {
            level_1: AggregateByLevel1Enum::Cluster,
            level_2: AggregateByLevel2Enum::Host,
        }
    }
}

impl AggregateBy {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("aggregate_by_grid")
            .num_columns(3)
            .show(ui, |ui| {
                ui.label("Aggregate by:");
                ui.label("Level 1:");
                let mut on_change_level_1 = false;
                on_change_level_1 |= ui
                    .radio_value(&mut self.level_1, AggregateByLevel1Enum::Cluster, "Cluster")
                    .clicked();
                on_change_level_1 |= ui
                    .radio_value(&mut self.level_1, AggregateByLevel1Enum::Host, "Host")
                    .clicked();
                on_change_level_1 |= ui
                    .radio_value(&mut self.level_1, AggregateByLevel1Enum::Owner, "Owner")
                    .clicked();
                ui.end_row();

                if on_change_level_1 {
                    self.level_2 = match self.level_1 {
                        AggregateByLevel1Enum::Cluster => AggregateByLevel2Enum::Host,
                        AggregateByLevel1Enum::Host => AggregateByLevel2Enum::Owner,
                        AggregateByLevel1Enum::Owner => AggregateByLevel2Enum::None,
                    }
                }

                ui.label("");
                match self.level_1 {
                    AggregateByLevel1Enum::Cluster => {
                        ui.label("Level 2:");
                        ui.radio_value(&mut self.level_2, AggregateByLevel2Enum::Host, "Host");
                        ui.radio_value(&mut self.level_2, AggregateByLevel2Enum::Owner, "Owner");
                        ui.radio_value(&mut self.level_2, AggregateByLevel2Enum::None, "None");
                    }
                    AggregateByLevel1Enum::Owner => {}
                    AggregateByLevel1Enum::Host => {
                        ui.label("Level 2:");
                        ui.radio_value(&mut self.level_2, AggregateByLevel2Enum::Owner, "Owner");
                        ui.radio_value(&mut self.level_2, AggregateByLevel2Enum::None, "None");
                    }
                }
                ui.end_row();
            });
    }
}
