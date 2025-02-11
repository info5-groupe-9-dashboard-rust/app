use super::view::View;
use crate::models::{application_context::ApplicationContext, filters::JobFilters, job::State};
use eframe::egui::{self, Grid, RichText};
use egui::TextEdit;
use strum::IntoEnumIterator;

pub struct Filtering {}

impl Default for Filtering {
    fn default() -> Self {
        Filtering {}
    }
}

impl Filtering {
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
        let mut owners_input = app
            .filters
            .owners
            .as_ref()
            .map_or(String::new(), |owners| owners.join(", "));

        if ui
            .add(TextEdit::singleline(&mut owners_input).desired_width(200.0))
            .changed()
        {
            let owners: Vec<String> = owners_input
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            app.filters.set_owners(owners);
        }
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

impl View for Filtering {
    fn render(&mut self, ui: &mut egui::Ui, app: &mut ApplicationContext) {
        ui.heading(RichText::new(t!("app.filters.title")).strong());
        ui.add_space(8.0);

        // Render each filter section separately
        self.render_job_id_range(ui, app);
        ui.add_space(10.0);

        self.render_owners_selector(ui, app);
        ui.add_space(10.0);

        self.render_states_selector(ui, app);
        ui.add_space(10.0);

        // Buttons
        ui.horizontal(|ui| {
            // if ui.button(t!("app.filters.apply")).clicked() {
            // app.filter_jobs();
            //}
            if ui.button(t!("app.filters.reset")).clicked() {
                app.filters = JobFilters::default();
            }
        });
    }
}
