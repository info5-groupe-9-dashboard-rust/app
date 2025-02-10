use eframe::egui;
use egui_extras::{Column, TableBuilder}; 
use egui::{Ui, Sense, RichText};

use crate::models::job::Job;

use super::job_details::JobDetailsWindow;

pub struct JobTable {
    page: usize,
    jobs_per_page: usize,
    details_window: Vec<JobDetailsWindow>,
}

impl Default for JobTable {
    fn default() -> Self {
        JobTable { 
            page: 0,
            jobs_per_page: 20,
            details_window: Vec::new(),
        }
    }
}

impl JobTable {
    pub fn ui(&mut self, ui: &mut Ui, jobs: &Vec<Job>) {

        egui::CentralPanel::default().show(ui.ctx(), |ui| {
            ui.add_space(10.0);
            ui.heading(RichText::new(t!("app.job_table.title")).strong().size(20.0));
            ui.add_space(8.0);

            let start_idx = self.page * self.jobs_per_page;
            let end_idx = (start_idx + self.jobs_per_page).min(jobs.len());
            let total_pages = (jobs.len() as f32 / self.jobs_per_page as f32).ceil() as usize;

            ui.horizontal(|ui| {
                if ui.button(RichText::new(t!("app.job_table.previous")).size(14.0)).clicked() && self.page > 0 {
                    self.page -= 1;
                }
                ui.label(RichText::new(format!("Page {} / {}", self.page + 1, total_pages)).size(14.0));
                if ui.button(RichText::new(t!("app.job_table.next")).size(14.0)).clicked() && self.page < total_pages - 1 {
                    self.page += 1;
                }
            });

            ui.separator();

            // Table avec pagination
            TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .sense(Sense::click())
                .column(Column::remainder().at_least(5.0).resizable(true))
                .column(Column::remainder().at_least(5.0).resizable(true))
                .column(Column::remainder().at_least(20.0).resizable(true))
                .column(Column::remainder().at_least(5.0).resizable(true))
                .column(Column::remainder().resizable(true))
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.label(RichText::new("Job ID").strong());
                    });
                    header.col(|ui| {
                        ui.label(RichText::new("Owner").strong());
                    });
                    header.col(|ui| {
                        ui.label(RichText::new("State").strong());
                    });
                    header.col(|ui| {
                        ui.label(RichText::new("Scheduled Start Time").strong());
                    });
                    header.col(|ui| {
                        ui.label(RichText::new("Wall Time").strong());
                    });
                })
                .body(|mut body| {
                    // N'afficher que les jobs de la page courante
                    for job in jobs[start_idx..end_idx].iter() {
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                if ui.button(job.id.to_string()).clicked() {
                                    self.details_window.push(JobDetailsWindow::new(job.clone()));
                                }
                            });
                            row.col(|ui| {
                                ui.label(job.owner.to_string());
                            });
                            row.col(|ui| {
                                ui.label(job.state.to_string());
                            });
                            row.col(|ui| {
                                ui.label(job.scheduled_start.to_string());
                            });
                            row.col(|ui| {
                                ui.label(job.walltime.to_string());
                            });
                        });
                    }
                });
                ui.add_space(10.0);
        });
        self.details_window.retain(|w| w.is_open());

        // Affichage des fenêtres de détails
        for window in self.details_window.iter_mut() {
            window.ui(ui);
        }
    }
}