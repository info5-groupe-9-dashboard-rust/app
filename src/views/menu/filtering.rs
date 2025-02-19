use std::cmp::Ordering;

use crate::models::data_structure::{
    application_context::ApplicationContext, cluster::Cluster, cpu::Cpu, filters::JobFilters,
    host::Host, job::JobState,
};
use eframe::egui::{self, Grid, RichText};
use egui::{ScrollArea, TextEdit};
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
                .default_size([600.0, 500.0])
                .show(ui.ctx(), |ui| {
                    ui.heading("Filter Options");

                    ui.separator(); // Add a separator

                    // Render the job id range
                    self.render_job_id_range(ui);
                    ui.add_space(10.0);

                    egui::CollapsingHeader::new("Owners")
                        .default_open(false)
                        .show(ui, |ui| {
                            self.render_owners_selector(ui, app);
                        });
                    ui.add_space(10.0);

                    egui::CollapsingHeader::new("Job States")
                        .default_open(false)
                        .show(ui, |ui| {
                            self.render_states_selector(ui);
                        });
                    ui.add_space(10.0);

                    ui.menu_button("Clusters", |ui| {
                        self.render_cluster_menu(ui, app);
                    });

                    ui.add_space(20.0);

                    ui.horizontal(|ui| {
                        if ui.button(t!("app.filters.apply")).clicked() {
                            app.filters = JobFilters::copy(&self.temp_filters); // add the temporary filters to the app filters
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

    fn render_cluster_menu(&mut self, ui: &mut egui::Ui, app: &mut ApplicationContext) {
        ui.set_max_width(124.0);

        for cluster in &app.all_clusters {
            ui.horizontal(|ui| {
                let mut is_selected = self
                    .temp_filters
                    .clusters
                    .as_ref()
                    .map_or(false, |clusters| {
                        clusters.iter().any(|c| c.name == cluster.name)
                    });

                if ui.checkbox(&mut is_selected, "").changed() {
                    if is_selected {
                        if let Some(clusters) = &mut self.temp_filters.clusters {
                            clusters.push(cluster.clone());
                        } else {
                            self.temp_filters.clusters = Some(vec![cluster.clone()]);
                        }
                    } else {
                        if let Some(clusters) = &mut self.temp_filters.clusters {
                            clusters.retain(|c| c.name != cluster.name);
                        }
                    }
                }

                ui.label(&cluster.name);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.menu_button(" ", |ui| {
                        self.render_host_menu(ui, cluster);
                    });
                });
            });
        }
    }

    fn render_host_menu(&mut self, ui: &mut egui::Ui, cluster: &Cluster) {
        ui.set_max_width(300.0);

        let mut hosts = cluster.hosts.clone();
        hosts.sort_by(|a, b| compare_host_names(&a.name, &b.name));

        let mut selected_cluster = self
            .temp_filters
            .clusters
            .as_mut()
            .and_then(|clusters| clusters.iter_mut().find(|c| c.name == cluster.name));

        if let Some(cluster) = selected_cluster.as_mut() {
            if ui.button("Deselect All").clicked() {
                cluster.hosts.clear();
            }
        }

        ScrollArea::vertical()
            .min_scrolled_height(50.0)
            .max_height(250.0)
            .show(ui, |ui| {
                for host in hosts {
                    let mut is_selected = selected_cluster
                        .as_ref()
                        .map_or(false, |c| c.hosts.iter().any(|h| h.name == host.name));

                    if ui.checkbox(&mut is_selected, &host.name).changed() {
                        if let Some(cluster) = selected_cluster.as_mut() {
                            if is_selected {
                                cluster.hosts.push(host.clone());
                            } else {
                                cluster.hosts.retain(|h| h.name != host.name);
                            }
                        }
                    }
                }
            });
    }
}

fn extract_number(s: &str) -> Option<u32> {
    s.split('-').nth(1)?.split('.').next()?.parse().ok()
}

fn compare_host_names(a: &str, b: &str) -> Ordering {
    match (extract_number(a), extract_number(b)) {
        (Some(num_a), Some(num_b)) => num_a.cmp(&num_b),
        _ => a.cmp(b),
    }
}
