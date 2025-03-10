use super::job_table_sorting::SortKey;
use egui::Layout;
use std::collections::BTreeMap;

/**
 * Struct for the column information
 */
pub struct ColumnInfo {
    pub name: String, // Name of the column
    pub selected: bool, // True if the column is selected
    pub sort_key: SortKey, // Sort key for the column
}

/**
 * Struct for the column selection
 */
pub struct ColumnSelection {
    pub values: BTreeMap<u8, ColumnInfo>, // Map of column index to column info
}

/**
 * Default implementation of the column selection
 */
impl Default for ColumnSelection {
    fn default() -> Self {
        let mut instance = Self {
            values: BTreeMap::new(),
        };
        instance.select_default();
        instance
    }
}

/**
 * Implementation of the column selection
 */
impl ColumnSelection {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            egui::Grid::new("")
                .num_columns(2)
                .striped(false)
                .show(ui, |ui| {
                    ui.with_layout(Layout::top_down(egui::Align::Max), |ui| {
                        ui.set_max_width(100.0);
                        ui.label(t!("app.job_table.table.columns"));
                    });
                    // Column selection grid
                    egui::Grid::new("column_selection_grid")
                        .num_columns(3)
                        .spacing([20.0, 8.0])
                        .show(ui, |ui| {
                            let mut count = 0;
                            for (_column, value) in self.values.iter_mut() {
                                ui.checkbox(&mut value.selected, t!(value.name.to_string()));
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
                        if ui.button(t!("app.job_table.select_all")).clicked() {
                            self.select_all();
                        }
                        if ui.button(t!("app.job_table.select_default")).clicked() {
                            self.select_default();
                        }
                    });
                });
        });
    }

    fn select_default(&mut self) {
        self.values.insert(
            0,
            ColumnInfo {
                name: "app.job_table.table.job_id".to_string(),
                selected: true,
                sort_key: SortKey::Id,
            },
        );
        self.values.insert(
            1,
            ColumnInfo {
                name: "app.job_table.table.owner".to_string(),
                selected: true,
                sort_key: SortKey::Owner,
            },
        );
        self.values.insert(
            2,
            ColumnInfo {
                name: "app.job_table.table.queue".to_string(),
                selected: false,
                sort_key: SortKey::Queue,
            },
        );
        self.values.insert(
            3,
            ColumnInfo {
                name: "app.job_table.table.command".to_string(),
                selected: false,
                sort_key: SortKey::Command,
            },
        );
        self.values.insert(
            4,
            ColumnInfo {
                name: "app.job_table.table.state".to_string(),
                selected: true,
                sort_key: SortKey::State,
            },
        );
        self.values.insert(
            5,
            ColumnInfo {
                name: "app.job_table.table.message".to_string(),
                selected: false,
                sort_key: SortKey::Message,
            },
        );
        self.values.insert(
            6,
            ColumnInfo {
                name: "app.job_table.table.submission_time".to_string(),
                selected: false,
                sort_key: SortKey::SubmissionTime,
            },
        );
        self.values.insert(
            7,
            ColumnInfo {
                name: "app.job_table.table.scheduled_start_time".to_string(),
                selected: true,
                sort_key: SortKey::ScheduledStartTime,
            },
        );
        self.values.insert(
            8,
            ColumnInfo {
                name: "app.job_table.table.start_time".to_string(),
                selected: false,
                sort_key: SortKey::StartTime,
            },
        );
        self.values.insert(
            9,
            ColumnInfo {
                name: "app.job_table.table.stop_time".to_string(),
                selected: false,
                sort_key: SortKey::StopTime,
            },
        );
        self.values.insert(
            10,
            ColumnInfo {
                name: "app.job_table.table.wall_time".to_string(),
                selected: true,
                sort_key: SortKey::WallTime,
            },
        );
        self.values.insert(
            11,
            ColumnInfo {
                name: "app.job_table.table.exit_code".to_string(),
                selected: false,
                sort_key: SortKey::ExitCode,
            },
        );
        self.values.insert(
            12,
            ColumnInfo {
                name: "app.job_table.table.clusters".to_string(),
                selected: false,
                sort_key: SortKey::Clusters,
            },
        );
    }

    fn select_all(&mut self) {
        for value in self.values.values_mut() {
            value.selected = true;
        }
    }
}
