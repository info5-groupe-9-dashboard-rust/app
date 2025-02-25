use std::collections::BTreeMap;
use egui::Layout;
use super::job_table_sorting::SortKey;

pub struct ColumnInfo {
    pub name: String,
    pub selected: bool,
    pub sort_key: SortKey
}

pub struct ColumnSelection {
    pub values: BTreeMap<u8, ColumnInfo>
}

impl Default for ColumnSelection {
    fn default() -> Self {
        let mut instance = Self {
            values: BTreeMap::new()
        };
        instance.select_default();
        instance
    }
}

impl ColumnSelection {

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {

            egui::Grid::new("").num_columns(2).striped(false).show(ui, |ui| {
                ui.with_layout(Layout::top_down(egui::Align::Max), |ui| {
                    ui.set_max_width(100.0);
                    ui.label("Columns:");
                });
                // Column selection grid
                egui::Grid::new("column_selection_grid")
                    .num_columns(3)
                    .spacing([20.0, 8.0])
                    .show(ui, |ui| {
                        let mut count = 0;
                        for (_column, value) in self.values.iter_mut() {
                            ui.checkbox(&mut value.selected, value.name.to_string());
                            count += 1;
                            if count % 3 == 0 {
                                ui.end_row();
                            }
                        }
                        if count % 3 != 0 {
                            ui.end_row();
                        }
                    });
                ui.end_row();
                ui.label("");
                ui.horizontal(|ui| {
                    if ui.button("Select All").clicked() {
                        self.select_all();
                    }
                    if ui.button("Default").clicked() {
                        self.select_default();
                    }
                });
            });
        });
    }

    fn select_default(&mut self) {
        // Please respect the order of the columns
        self.values.insert(0, ColumnInfo { name: "Job ID".to_string(), selected: true, sort_key: SortKey::Id });
        self.values.insert(1, ColumnInfo { name: "Owner".to_string(), selected: true, sort_key: SortKey::Owner });
        self.values.insert(2, ColumnInfo { name: "Queue".to_string(), selected: false, sort_key: SortKey::Queue });
        self.values.insert(3, ColumnInfo { name: "Command".to_string(), selected: false, sort_key: SortKey::Command });
        self.values.insert(4, ColumnInfo { name: "State".to_string(), selected: true, sort_key: SortKey::State });
        self.values.insert(5, ColumnInfo { name: "Message".to_string(), selected: false, sort_key: SortKey::Message });
        self.values.insert(6, ColumnInfo { name: "Submission Time".to_string(), selected: false, sort_key: SortKey::SubmissionTime });
        self.values.insert(7, ColumnInfo { name: "Scheduled Start Time".to_string(), selected: true, sort_key: SortKey::ScheduledStartTime });
        self.values.insert(8, ColumnInfo { name: "Start Time".to_string(), selected: false, sort_key: SortKey::StartTime });
        self.values.insert(9, ColumnInfo { name: "Stop Time".to_string(), selected: false, sort_key: SortKey::StopTime });
        self.values.insert(10, ColumnInfo { name: "Wall Time".to_string(), selected: true, sort_key: SortKey::WallTime });
        self.values.insert(11, ColumnInfo { name: "Exit Code".to_string(), selected: false, sort_key: SortKey::ExitCode });
        self.values.insert(12, ColumnInfo { name: "Clusters".to_string(), selected: false, sort_key: SortKey::Clusters });
    }

    fn select_all(&mut self) {
        for value in self.values.values_mut() {
            value.selected = true;
        }
    }
}