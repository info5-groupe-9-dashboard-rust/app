use crate::models::utils::date_converter::format_timestamp;
use crate::models::data_structure::job::Job;
use eframe::egui;

pub struct JobDetailsWindow {
    open: bool,
    job: Job,
}

impl JobDetailsWindow {
    pub fn new(job: Job) -> Self {
        Self {
            open: true,
            job: job,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // If the window is not open, do not render it
        // This is useful to avoid rendering the window when it is not needed
        if !self.open {
            return;
        }

        egui::Window::new(format!("Job Details: {}", self.job.id))
            .collapsible(true)
            .movable(true)
            .open(&mut self.open)
            .auto_sized()
            .show(ui.ctx(), |ui| {
                // Base information
                ui.group(|ui| {
                    ui.heading("Basic Information");
                    ui.horizontal(|ui| {
                        ui.label("Owner: ");
                        ui.strong(self.job.owner.to_string());
                    });
                    ui.horizontal(|ui| {
                        ui.label("Queue: ");
                        ui.strong(self.job.queue.to_string());
                    });
                    ui.horizontal(|ui| {
                        ui.label("Command: ");
                        ui.strong(self.job.command.to_string());
                    });
                });

                ui.add_space(8.0);

                // Status
                ui.group(|ui| {
                    ui.heading("Status");
                    ui.horizontal(|ui| {
                        ui.label("State: ");
                        let state_text = self.job.state.get_label();
                        let (state_color, bg_color) = self.job.state.get_color();
                        ui.label(
                            egui::RichText::new(state_text)
                                .color(state_color)
                                .background_color(bg_color)
                                .strong(),
                        );
                    });
                    if let Some(exit_code) = self.job.exit_code {
                        ui.horizontal(|ui| {
                            ui.label("Exit Code: ");
                            ui.strong(exit_code.to_string());
                        });
                    }
                    if let Some(message) = &self.job.message {
                        ui.horizontal(|ui| {
                            ui.label("Message: ");
                            ui.label(message);
                        });
                    }
                });

                ui.add_space(8.0);

                // Timing information
                ui.group(|ui| {
                    ui.heading("Timing Information");

                    ui.horizontal(|ui| {
                        ui.label("Submission Time: ");
                        ui.strong(format_timestamp(self.job.submission_time));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Scheduled Start: ");
                        ui.strong(format_timestamp(self.job.scheduled_start));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Actual Start: ");
                        ui.strong(format_timestamp(self.job.start_time));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Stop Time: ");
                        ui.strong(format_timestamp(self.job.stop_time));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Walltime: ");
                        ui.strong(format!("{} seconds", self.job.walltime));
                    });
                });

                ui.add_space(8.0);

                // Ressources
                ui.group(|ui| {
                    ui.heading("Resources");
                    ui.horizontal(|ui| {
                        ui.label("Assigned Resources: ");
                        ui.label(format!("{:?}", self.job.assigned_resources));
                    });
                });

                ui.add_space(8.0);
            });
    }

    pub fn is_open(&self) -> bool {
        self.open
    }
}
