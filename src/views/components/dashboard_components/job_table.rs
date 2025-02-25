use crate::models::data_structure::application_context::ApplicationContext;
use crate::models::utils::date_converter::format_timestamp;
use crate::models::utils::utils::get_tree_structure_for_job;
use crate::{models::data_structure::job::Job, views::components::job_details::JobDetailsWindow};
use eframe::egui;
use egui::{RichText, Sense, Ui};
use egui_extras::{Column, TableBuilder};

pub struct JobTable {
    page: usize,
    jobs_per_page: usize,
    details_window: Vec<JobDetailsWindow>,
    start_idx: usize,
    end_idx: usize,
    displayed_jobs_per_page: Vec<Job>,
    sort_key: SortKey,
    sort_ascending: bool
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
            sort_ascending: true
        }
    }
}

#[derive(PartialEq)]
enum SortKey {
    Id,
    Owner,
    State,
    StartTime,
    WallTime,
}

impl JobTable {
    pub fn ui(&mut self, ui: &mut Ui, app: &mut ApplicationContext) {
        self.displayed_jobs_per_page = app.filtered_jobs.clone();
        self.sort_jobs();

        ui.add_space(10.0);
        ui.heading(RichText::new(t!("app.job_table.title")).strong().size(20.0));
        ui.add_space(8.0);

        self.start_idx = self.page * self.jobs_per_page;
        self.end_idx = (self.start_idx + self.jobs_per_page).min(app.filtered_jobs.len());
        let total_pages =
            (app.filtered_jobs.len() as f32 / self.jobs_per_page as f32).ceil() as usize;

        // println!(
        //     "start_idx: {}, end_idx: {}, total_pages: {}, app.filtered_jobs len {}",
        //     self.start_idx,
        //     self.end_idx,
        //     total_pages,
        //     app.filtered_jobs.len()
        // );

        if self.start_idx >= app.filtered_jobs.len() {
            self.reset_pagination();
            return;
        }

        ui.horizontal(|ui| {
            if ui
                .button(RichText::new(t!("app.job_table.previous")).size(14.0))
                .clicked()
                && self.page > 0
            {
                self.page -= 1;
            }
            ui.label(RichText::new(format!("Page {} / {}", self.page + 1, total_pages)).size(14.0));
            if ui
                .button(RichText::new(t!("app.job_table.next")).size(14.0))
                .clicked()
                && self.page < total_pages - 1
            {
                self.page += 1;
            }
        });

        ui.separator();

        // Table with pagination and sorting
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .sense(Sense::click())
            .column(Column::auto().at_least(2.0).resizable(true))
            .column(Column::remainder().at_least(5.0).resizable(true))
            .column(Column::remainder().at_least(5.0).resizable(true))
            .column(Column::remainder().at_least(20.0).resizable(true))
            .column(Column::remainder().at_least(5.0).resizable(true))
            .column(Column::remainder().resizable(true))
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.label(RichText::new(t!("app.job_table.table.row")).strong());
                });

                header.col(|ui| {
                    if ui
                        .button(RichText::new(t!("app.job_table.table.job_id")).strong())
                        .clicked()
                    {
                        self.sort_key = SortKey::Id;
                        self.sort_ascending = !self.sort_ascending;
                        self.page = 0;
                    }
                });

                header.col(|ui| {
                    if ui
                        .button(RichText::new(t!("app.job_table.table.owner")).strong())
                        .clicked()
                    {
                        self.sort_key = SortKey::Owner;
                        self.sort_ascending = !self.sort_ascending;
                        self.page = 0;
                    }
                });
                header.col(|ui| {
                    if ui
                        .button(RichText::new(t!("app.job_table.table.state")).strong())
                        .clicked()
                    {
                        self.sort_key = SortKey::State;
                        self.sort_ascending = !self.sort_ascending;
                        self.page = 0;
                    }
                });
                header.col(|ui| {
                    if ui
                        .button(RichText::new(t!("app.job_table.table.start_time")).strong())
                        .clicked()
                    {
                        self.sort_key = SortKey::StartTime;
                        self.sort_ascending = !self.sort_ascending;
                        self.page = 0;
                    }
                });
                header.col(|ui| {
                    if ui
                        .button(RichText::new(t!("app.job_table.table.walltime")).strong())
                        .clicked()
                    {
                        self.sort_key = SortKey::WallTime;
                        self.sort_ascending = !self.sort_ascending;
                        self.page = 0;
                    }
                });
            })
            .body(|mut body| {
                for job in self.displayed_jobs_per_page[self.start_idx..self.end_idx].iter() {
                    body.row(20.0, |mut row| {
                    
                        // Row index
                        let row_index = self.start_idx + row.index() + 1;
                        row.col(|ui| {
                            ui.label(row_index.to_string());
                        });

                        // Job ID
                        row.col(|ui| {
                            ui.label(job.id.to_string());
                        });

                        // Owner
                        row.col(|ui| {
                            ui.label(job.owner.to_string());
                        });

                        // State
                        row.col(|ui| {
                            let state_text = job.state.get_label();
                            let (state_color, bg_color) = job.state.get_color();
                            ui.label(
                                egui::RichText::new(state_text)
                                    .color(state_color)
                                    .background_color(bg_color)
                                    .strong(),
                            );
                        });

                        // Start time
                        row.col(|ui| {
                            ui.label(format_timestamp(job.start_time));
                        });

                        // Walltime
                        row.col(|ui| {
                            ui.label(format_timestamp(job.start_time + job.walltime));
                        });

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

    fn sort_jobs(&mut self) {
        match self.sort_key {
            SortKey::Id => {
                if self.sort_ascending {
                    self.displayed_jobs_per_page.sort_by(|a, b| a.id.cmp(&b.id));
                } else {
                    self.displayed_jobs_per_page.sort_by(|a, b| b.id.cmp(&a.id));
                }
            }
            SortKey::Owner => {
                if self.sort_ascending {
                    self.displayed_jobs_per_page
                        .sort_by(|a, b| a.owner.cmp(&b.owner));
                } else {
                    self.displayed_jobs_per_page
                        .sort_by(|a, b| b.owner.cmp(&a.owner));
                }
            }
            SortKey::State => {
                if self.sort_ascending {
                    self.displayed_jobs_per_page
                        .sort_by(|a, b| a.state.cmp(&b.state));
                } else {
                    self.displayed_jobs_per_page
                        .sort_by(|a, b| b.state.cmp(&a.state));
                }
            }
            SortKey::StartTime => {
                if self.sort_ascending {
                    self.displayed_jobs_per_page
                        .sort_by(|a, b| a.scheduled_start.cmp(&b.scheduled_start));
                } else {
                    self.displayed_jobs_per_page
                        .sort_by(|a, b| b.scheduled_start.cmp(&a.scheduled_start));
                }
            }
            SortKey::WallTime => {
                if self.sort_ascending {
                    self.displayed_jobs_per_page
                        .sort_by(|a, b| a.walltime.cmp(&b.walltime));
                } else {
                    self.displayed_jobs_per_page
                        .sort_by(|a, b| b.walltime.cmp(&a.walltime));
                }
            }
        }
    }
}
