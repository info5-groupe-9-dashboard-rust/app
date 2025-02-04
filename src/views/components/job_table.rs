use eframe::egui;
use egui_extras::{Column, TableBuilder};
use egui::{Ui, Sense, RichText};

use crate::models::job::Job;

pub struct JobTable {
    jobs: Vec<Job>,
}

impl JobTable {

    pub fn new(jobs: &Vec<Job>) -> Self {
        JobTable { jobs: jobs.clone() }
    }

    pub fn ui(&self, ui: &mut Ui) {
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

                /* Job ID (header) */
                header.col(|ui| {
                    ui.label(RichText::new("Job ID").strong());
                });

                /* Owner (header) */
                header.col(|ui| {
                    ui.label(RichText::new("Owner").strong());
                });

                /* State (header) */
                header.col(|ui| {
                    ui.label(RichText::new("State").strong());
                });

                /* Scheduled Start Time (header) */
                header.col(|ui| {
                    ui.label(RichText::new("Scheduled Start Time").strong());
                });

                /* Wall Time (header) */
                header.col(|ui| {
                    ui.label(RichText::new("Wall Time").strong());
                });
            })
            .body(|mut body| {
                for job in &self.jobs {
                    body.row(20.0, |mut row| {

                        /* Job ID (value) */
                        row.col(|ui| {
                            if ui.button(job.id.to_string()).clicked() { // Display job details when clicking on the job ID
                                println!("----------------------------------------");
                                job._display();
                            }
                        });

                        /* Owner (value) */
                        row.col(|ui| {
                            ui.label(job.owner.to_string());
                        });

                        /* State (value) */
                        row.col(|ui| {
                            ui.label(job.state.to_string());
                        });

                        /* Scheduled Start Time (value) */
                        row.col(|ui| {
                            ui.label(job.scheduled_start.to_string());
                        });

                        /* Wall Time (value) */
                        row.col(|ui| {
                            ui.label(job.walltime.to_string());
                        });
                    });
                }
            });
    }
}
