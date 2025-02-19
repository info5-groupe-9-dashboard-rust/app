use eframe::egui;
use super::{metric_box::MetricBox, metric_chart::MetricChart};
use eframe::egui;

pub struct MetricGrid {
    columns: usize,
    spacing: f32,
}

impl Default for MetricGrid {
    fn default() -> Self {
        Self {
            columns: 3,
            spacing: 10.0,
        }
    }
}

impl MetricGrid {
    pub fn show<F>(&self, ui: &mut egui::Ui, add_contents: F)
    where
        F: FnOnce(&mut MetricGridBuilder),
    {
        let available_width = ui.available_width();

        // First calculate the total minimum width needed
        let min_total_width = (MetricBox::MIN_WIDTH * self.columns as f32)
            + (self.spacing * (self.columns - 1) as f32);

        // If available width is less than minimum, use minimum
        let actual_width = available_width.max(min_total_width);

        // Recalculate column width taking spacing into account
        let column_width =
            (actual_width - (self.spacing * (self.columns - 1) as f32)) / self.columns as f32;

        egui::Grid::new("metrics_grid")
            .spacing([self.spacing, self.spacing])
            .min_col_width(column_width)
            .max_col_width(column_width) // Ajouter une largeur maximale
            .show(ui, |ui| {
                let mut builder = MetricGridBuilder {
                    ui,
                    column_width,
                    current_column: 0,
                    columns: self.columns,
                };
                add_contents(&mut builder);

                if builder.current_column > 0 && builder.current_column < builder.columns {
                    builder.new_row();
                }
            });
    }
}

pub struct MetricGridBuilder<'a> {
    ui: &'a mut egui::Ui,
    column_width: f32,
    current_column: usize,
    columns: usize,
}

impl<'a> MetricGridBuilder<'a> {
    pub fn add_metric(&mut self, metric: MetricBox) {
        if self.current_column >= self.columns {
            self.new_row();
        }

        self.ui
            .add_sized([self.column_width, MetricBox::MIN_HEIGHT], metric);
        self.current_column += 1;
    }

    pub fn add_chart(&mut self, chart: MetricChart) {
        if self.current_column >= self.columns {
            self.new_row();
        }

        self.ui
            .add_sized([self.column_width, MetricBox::MIN_HEIGHT], chart);
        self.current_column += 1;
    }

    pub fn new_row(&mut self) {
        if self.current_column > 0 {
            self.ui.end_row();
        }
        self.current_column = 0;
    }
}
