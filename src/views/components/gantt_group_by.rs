#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GroupByEnum {
    Owner,
    Cluster,
    Host
}

pub struct GroupBy {
    pub value: GroupByEnum
}

impl Default for GroupBy {
    fn default() -> Self {
        Self {
            value: GroupByEnum::Owner,
        }
    }
}

impl GroupBy {

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Group by:");
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.value, GroupByEnum::Cluster, "Cluster");
                ui.radio_value(&mut self.value, GroupByEnum::Host, "Host");
                ui.radio_value(&mut self.value, GroupByEnum::Owner, "Owner");
            });
        });
    }
}