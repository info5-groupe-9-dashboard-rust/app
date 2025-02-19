use crate::models::data_structure::{
    application_context::ApplicationContext, filters::JobFilters, job::JobState,
};
use eframe::egui::{self, Grid, RichText};
use egui::TextEdit;
use strum::IntoEnumIterator;

pub struct Filtering {
    open: bool,
    temp_filters: JobFilters,
}

impl Default for Filtering {
    fn default() -> Self {
        Filtering {
            open: false,
            temp_filters: JobFilters::default(),
        }
    }
}

impl Filtering {
    pub fn open(&mut self) {
        self.open = true;
    }

    // If the window is open, render the filters
    pub fn ui(&mut self, ui: &mut egui::Ui, app: &mut ApplicationContext) {
        let mut open = self.open; // Copy the value of self.open to a mutable variable
        if self.open {
            egui::Window::new("Filters")
                .collapsible(true)
                .movable(true)
                .open(&mut open)
                .default_size([400.0, 500.0])
                .show(ui.ctx(), |ui| {
                    ui.heading("Filter Options");

                    ui.separator(); // Add a separator

                    // Render the job id range
                    self.render_job_id_range(ui);
                    ui.add_space(10.0);

                    self.render_owners_selector(ui, app);
                    ui.add_space(10.0);

                    self.render_states_selector(ui);

                    ui.add_space(20.0);

                    ui.horizontal(|ui| {
                        if ui.button(t!("app.filters.apply")).clicked() {
                            app.filters = JobFilters::copy(&self.temp_filters); // add the temporary filters to the app filters
                            println!("Applying filters: {:?}", app.filters);
                            app.filter_jobs(); // Filter the jobs
                            self.open = false; // Close the window
                        }
                        if ui.button(t!("app.filters.reset")).clicked() {
                            self.reset_filters(); // Reset the filters
                            app.filters = JobFilters::default();
                        }
                    });
                });
        }
        self.open = open;
    }

    pub fn reset_filters(&mut self) {
        self.temp_filters = JobFilters::default();
    }

    fn render_job_id_range(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new(t!("Job Id")).strong());
        ui.horizontal(|ui| {
            let mut start_id = self
                .temp_filters
                .job_id_range
                .map(|(s, _)| s)
                .unwrap_or(0)
                .to_string();
            let mut end_id = self
                .temp_filters
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
                    self.temp_filters.set_job_id_range(start, end);
                }
            }

            ui.label(RichText::new(t!("app.filters.to")).strong());
            if ui
                .add(TextEdit::singleline(&mut end_id).desired_width(50.0))
                .changed()
            {
                if let (Ok(start), Ok(end)) = (start_id.parse(), end_id.parse()) {
                    self.temp_filters.set_job_id_range(start, end);
                }
            }
        });
    }

    fn render_owners_selector(&mut self, ui: &mut egui::Ui, app: &mut ApplicationContext) {
        ui.label(RichText::new(t!("Owners")).strong());

        let unique_owners = app.get_unique_owners();
        let mut selected_owners = self.temp_filters.owners.clone().unwrap_or_default();

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
                        self.temp_filters.set_owners(if selected_owners.is_empty() {
                            None
                        } else {
                            Some(selected_owners.clone())
                        });
                    }
                    if i % 2 == 1 {
                        ui.end_row();
                    }
                }
            });
    }

    fn render_states_selector(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new(t!("app.filters.states")).strong());

        let mut selected_states = self.temp_filters.states.clone().unwrap_or_default();

        Grid::new("states_grid")
            .num_columns(2)
            .spacing([10.0, 5.0])
            .show(ui, |ui| {
                for (i, state) in JobState::iter().enumerate() {
                    let mut is_selected = selected_states.contains(&state);
                    if ui
                        .checkbox(&mut is_selected, format!("{:?}", state))
                        .changed()
                    {
                        if is_selected {
                            selected_states.push(state);
                        } else {
                            selected_states.retain(|s| s != &state);
                        }
                        self.temp_filters.set_states(if selected_states.is_empty() {
                            None
                        } else {
                            Some(selected_states.clone())
                        });
                    }
                    if i % 2 == 1 {
                        ui.end_row();
                    }
                }
            });
    }
}
