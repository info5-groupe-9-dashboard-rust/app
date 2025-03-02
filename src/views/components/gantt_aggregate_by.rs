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
                ui.label(format!("{}:", t!("app.gantt.settings.aggregate_by")));
                ui.label(format!("{} 1:", t!("app.gantt.settings.level")));
                let mut on_change_level_1 = false;
                on_change_level_1 |= ui
                    .radio_value(&mut self.level_1, AggregateByLevel1Enum::Cluster, "Cluster")
                    .clicked();
                on_change_level_1 |= ui
                    .radio_value(&mut self.level_1, AggregateByLevel1Enum::Host, t!("app.gantt.settings.host"))
                    .clicked();
                on_change_level_1 |= ui
                    .radio_value(&mut self.level_1, AggregateByLevel1Enum::Owner, t!("app.gantt.settings.owner"))
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
                        ui.label(format!("{} 2:", t!("app.gantt.settings.level")));
                        ui.radio_value(&mut self.level_2, AggregateByLevel2Enum::Host, t!("app.gantt.settings.host"));
                        ui.radio_value(&mut self.level_2, AggregateByLevel2Enum::Owner, t!("app.gantt.settings.owner"));
                        ui.radio_value(&mut self.level_2, AggregateByLevel2Enum::None, t!("app.gantt.settings.none"));
                    }
                    AggregateByLevel1Enum::Owner => {}
                    AggregateByLevel1Enum::Host => {
                        ui.label(format!("{} 2:", t!("app.gantt.settings.level")));
                        ui.radio_value(&mut self.level_2, AggregateByLevel2Enum::Owner, t!("app.gantt.settings.owner"));
                        ui.radio_value(&mut self.level_2, AggregateByLevel2Enum::None, t!("app.gantt.settings.none"));
                    }
                }
                ui.end_row();
            });
    }
}
