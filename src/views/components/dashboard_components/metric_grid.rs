use eframe::egui;
use super::metric_box::MetricBox;

pub struct MetricGrid {
    columns: usize,
    spacing: f32,
}

impl Default for MetricGrid {
    fn default() -> Self {
        Self {
            columns: 4,
            spacing: 12.0, 
        }
    }
}

impl MetricGrid {
    pub fn show<F>(&self, ui: &mut egui::Ui, add_contents: F)
    where
        F: FnOnce(&mut MetricGridBuilder),
    {
        let available_width = ui.available_width();
        
        // Calculer la largeur minimale nécessaire pour la grille
        let min_grid_width = (MetricBox::MIN_WIDTH * self.columns as f32) + 
            (self.spacing * (self.columns - 1) as f32);
        
        // Si la largeur disponible est inférieure au minimum, ajuster le nombre de colonnes
        let effective_columns = if available_width < min_grid_width {
            ((available_width + self.spacing) / (MetricBox::MIN_WIDTH + self.spacing))
                .floor()
                .max(1.0) as usize
        } else {
            self.columns
        };

        let total_spacing = self.spacing * (effective_columns - 1) as f32;
        let column_width = ((available_width - total_spacing) / effective_columns as f32)
            .max(MetricBox::MIN_WIDTH);

        egui::Grid::new("metrics_grid")
            .spacing([self.spacing, self.spacing])
            .min_col_width(column_width)
            .max_col_width(column_width)
            .show(ui, |ui| {
                let mut builder = MetricGridBuilder {
                    ui,
                    column_width,
                    current_column: 0,
                    columns: effective_columns,
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

        self.ui.add_sized([self.column_width, MetricBox::MIN_HEIGHT], metric);
        self.current_column += 1;
    }

    pub fn new_row(&mut self) {
        if self.current_column > 0 {
            self.ui.end_row();
        }
        self.current_column = 0;
    }
}