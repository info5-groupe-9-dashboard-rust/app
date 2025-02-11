use super::view::View;
use crate::models::{application_context::ApplicationContext, filters::JobFilters, job::State};
use eframe::egui::{self, Grid, RichText};
use egui::TextEdit;
use strum::IntoEnumIterator;

pub struct Filtering {
    open: bool,
}

impl Default for Filtering {
    fn default() -> Self {
        Filtering { open: false }
    }
}

impl Filtering {
    pub fn open(&mut self) {
        self.open = true;
    }

    // Si la popup est ouverte, on la dessine

    pub fn ui(&mut self, ui: &mut egui::Ui, app: &mut ApplicationContext) {
        let mut open = self.open; // Copier la valeur de self.open dans une variable locale
        if self.open {
            egui::Window::new("Filters")
                .collapsible(true)
                .movable(true)
                .open(&mut open)
                .default_size([400.0, 500.0])
                .show(ui.ctx(), |ui| {
                    ui.heading("Filter Options");

                    ui.separator(); // Ligne de séparation

                    // Appeler les fonctions de rendu des filtres ici
                    self.render_job_id_range(ui, app);
                    ui.add_space(10.0);

                    self.render_owners_selector(ui, app);
                    ui.add_space(10.0);

                    self.render_states_selector(ui, app);

                    ui.add_space(20.0);

                    ui.horizontal(|ui| {
                        if ui.button(t!("app.filters.reset")).clicked() {
                            app.filters = JobFilters::default();
                        }
                    });
                });
        }
        self.open = open;
    }

    fn render_job_id_range(&mut self, ui: &mut egui::Ui, app: &mut ApplicationContext) {
        ui.label(RichText::new(t!("Job Id")).strong());
        ui.horizontal(|ui| {
            let mut start_id = app
                .filters
                .job_id_range
                .map(|(s, _)| s)
                .unwrap_or(0)
                .to_string();
            let mut end_id = app
                .filters
                .job_id_range
                .map(|(_, e)| e)
                .unwrap_or(0)
                .to_string();

            ui.label(RichText::new(t!("app.filters.from")).strong());
            if ui
                .add(TextEdit::singleline(&mut start_id).desired_width(50.0))
                .changed()
            {
                if let (Ok(start), Ok(end)) = (start_id.parse(), end_id.parse()) {
                    app.filters.set_job_id_range(start, end);
                }
            }

            ui.label(RichText::new(t!("app.filters.to")).strong());
            if ui
                .add(TextEdit::singleline(&mut end_id).desired_width(50.0))
                .changed()
            {
                if let (Ok(start), Ok(end)) = (start_id.parse(), end_id.parse()) {
                    app.filters.set_job_id_range(start, end);
                }
            }
        });
    }

    fn render_owners_selector(&mut self, ui: &mut egui::Ui, app: &mut ApplicationContext) {
        ui.label(RichText::new(t!("Owners")).strong());

        let unique_owners = app.get_unique_owners();
        let mut selected_owners = app.filters.owners.clone().unwrap_or_default();

        Grid::new("owners_grid")
            .num_columns(2)
            .spacing([10.0, 5.0])
            .show(ui, |ui| {
                for (i, owner) in unique_owners.iter().enumerate() {
                    let mut is_selected = selected_owners.contains(owner);
                    if ui.checkbox(&mut is_selected, owner).changed() {
                        if is_selected {
                            selected_owners.push(owner.clone());
                        } else {
                            selected_owners.retain(|o| o != owner);
                        }
                        app.filters.set_owners(selected_owners.clone());
                    }
                    if i % 2 == 1 {
                        ui.end_row();
                    }
                }
            });
    }

    fn render_states_selector(&mut self, ui: &mut egui::Ui, app: &mut ApplicationContext) {
        ui.label(RichText::new(t!("app.filters.states")).strong());

        Grid::new("states_grid")
            .num_columns(2)
            .spacing([10.0, 5.0])
            .show(ui, |ui| {
                for (i, state) in State::iter().enumerate() {
                    let mut is_selected = app
                        .filters
                        .states
                        .as_ref()
                        .map_or(false, |states| states.contains(&state));
                    if ui
                        .checkbox(&mut is_selected, format!("{:?}", state))
                        .changed()
                    {
                        if is_selected {
                            app.filters.states.get_or_insert_with(Vec::new).push(state);
                        } else if let Some(states) = app.filters.states.as_mut() {
                            states.retain(|s| s != &state);
                        }
                    }
                    if i % 2 == 1 {
                        ui.end_row();
                    }
                }
            });
    }
}
