use crate::models::data_structure::application_context::ApplicationContext;
use crate::models::utils::date_converter::format_timestamp;
use crate::models::utils::utils::get_tree_structure_for_job;
use crate::{models::data_structure::job::Job, views::components::job_details::JobDetailsWindow};
use eframe::egui;
use egui::{RichText, Sense, Ui};
use egui_extras::{Column, TableBuilder};
use super::job_table_col_selection::ColumnSelection;
use super::job_table_sorting::SortKey;

pub struct JobTable {
    page: usize,
    jobs_per_page: usize,
    details_window: Vec<JobDetailsWindow>,
    start_idx: usize,
    end_idx: usize,
    displayed_jobs_per_page: Vec<Job>,
    sort_key: SortKey,
    sort_ascending: bool,
    column_selection: ColumnSelection
}

impl Default for JobTable {
    fn default() -> Self {
        JobTable {
            page: 0,
            jobs_per_page: 20,
            details_window: Vec::new(),
            start_idx: 0,
            end_idx: 0,
            displayed_jobs_per_page: Vec::new(),
            sort_key: SortKey::Id,
            sort_ascending: true,
            column_selection: ColumnSelection::default(),
        }
    }
}

impl JobTable {
    pub fn ui(&mut self, ui: &mut Ui, app: &mut ApplicationContext) {
        self.displayed_jobs_per_page = app.filtered_jobs.clone();
        self.sort_key.sort_jobs(&mut self.displayed_jobs_per_page, self.sort_ascending);

        ui.add_space(10.0);
        ui.heading(RichText::new(t!("app.job_table.title")).strong().size(20.0));
        ui.add_space(8.0);

        self.start_idx = self.page * self.jobs_per_page;
        self.end_idx = (self.start_idx + self.jobs_per_page).min(app.filtered_jobs.len());
        let total_pages =
            (app.filtered_jobs.len() as f32 / self.jobs_per_page as f32).ceil() as usize;

        if self.start_idx >= app.filtered_jobs.len() {
            self.reset_pagination();
            return;
        }

        ui.horizontal(|ui| {
            // Left side with Options
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.menu_button("🔧 Settings", |ui| {
                    ui.set_max_height(500.0);

                    // Column selection
                    self.column_selection.ui(ui);
                });
                ui.menu_button("❓", |ui| {
                    ui.label(
                        "Actions disponibles:\n\
                         - Navigation: Utilisez les boutons page suivante/précédente\n\
                         - Tri: Cliquez sur l'en-tête d'une colonne pour trier (croissant/décroissant)\n\
                         - Détails: Cliquez sur une ligne pour voir les détails du job\n\
                         - Paramètres: Via le bouton '🔧 Settings' pour configurer l'affichage",
                    );
                });
            });

            // Right side with Pagination, using remaining space
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let mut pagination_ui = |ui: &mut Ui| {
                    if ui
                    .button(RichText::new(t!("app.job_table.next")).size(14.0))
                    .clicked()
                    && self.page < total_pages - 1
                    {
                    self.page += 1;
                    }
                    ui.label(RichText::new(format!("Page {} / {}", self.page + 1, total_pages)).size(14.0));
                    if ui
                    .button(RichText::new(t!("app.job_table.previous")).size(14.0))
                    .clicked()
                    && self.page > 0
                    {
                    self.page -= 1;
                    }
                };
                pagination_ui(ui);
            });
        });

        ui.separator();

        egui::ScrollArea::horizontal().show(ui, |ui| {
            // Table with pagination, sorting and selection
        let available_width = ui.available_width();
        let mut table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .sense(Sense::click())
            .column(Column::auto().at_least(2.0).resizable(true));
            
        for value in self.column_selection.values.values() {
            if value.selected {
                table = table.column(Column::remainder().at_least(10.0).at_most(available_width).resizable(true));
            }
        }

        table
            .header(20.0, |mut header| {

                header.col(|ui| {
                    ui.label(RichText::new(t!("app.job_table.table.row")).strong());
                });

                for value in self.column_selection.values.values() {
                    if value.selected {
                        header.col(|ui| {
                            if ui
                                .button(RichText::new(t!(value.name.clone())).strong())
                                .clicked()
                            {
                                self.sort_key = value.sort_key;
                                self.sort_ascending = !self.sort_ascending;
                                self.page = 0;
                            }
                        });
                    }
                }
            })
            .body(|mut body| {
                for job in self.displayed_jobs_per_page[self.start_idx..self.end_idx].iter() {
                    body.row(20.0, |mut row| {
                    
                        // Row index
                        let row_index = self.start_idx + row.index() + 1;
                        row.col(|ui| {
                            ui.label(row_index.to_string());
                        });

                        for value in self.column_selection.values.values() {
                            if value.selected {
                                row.col(|ui| {
                                    match value.sort_key {
                                        SortKey::Id => {
                                            ui.label(job.id.to_string());
                                        }
                                        SortKey::Owner => {
                                            ui.label(job.owner.to_string());
                                        }
                                        SortKey::State => {
                                            let state_text = job.state.get_label();
                                            let (state_color, bg_color) = job.state.get_color();
                                            ui.label(
                                                egui::RichText::new(state_text)
                                                    .color(state_color)
                                                    .background_color(bg_color)
                                                    .strong(),
                                            );
                                        }
                                        SortKey::StartTime => {
                                            ui.label(format_timestamp(job.start_time));
                                        }
                                        SortKey::WallTime => {
                                            ui.label(job.walltime.to_string());
                                        }
                                        SortKey::Queue => {
                                            ui.label(&job.queue);
                                        }
                                        SortKey::Command => {
                                            ui.label(&job.command);
                                        }
                                        SortKey::Message => {
                                            ui.label(job.message.as_deref().unwrap_or(""));
                                        }
                                        SortKey::SubmissionTime => {
                                            ui.label(format_timestamp(job.submission_time));
                                        }
                                        SortKey::ScheduledStartTime => {
                                            ui.label(format_timestamp(job.scheduled_start));
                                        }
                                        SortKey::StopTime => {
                                            ui.label(format_timestamp(job.stop_time));
                                        }
                                        SortKey::ExitCode => {
                                            ui.label(job.exit_code.map_or("N/A".to_string(), |code| code.to_string()));
                                        }
                                        SortKey::Clusters => {
                                            ui.label(job.clusters.join(", "));
                                        }
                                    }
                                });
                            }
                        }

                        // Clickable row
                        let response = row.response().interact(Sense::click());
                        if response.clicked() {
                            for window in self.details_window.iter_mut() {
                                if window.job.id == job.id {
                                    window.open = true;
                                    return;
                                }
                            }
                            self.details_window.push(JobDetailsWindow::new(
                                job.clone(),
                                get_tree_structure_for_job(job, &app.all_clusters),
                            ));
                        }

                    });
                }
            });
        });
        
        ui.add_space(10.0);

        for window in self.details_window.iter_mut() {
            window.ui(ui);
        }
    }

    pub fn reset_pagination(&mut self) {
        self.page = 0;
        self.start_idx = 0;
        self.end_idx = 0;
    }
}
