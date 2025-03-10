use crate::models::data_structure::cluster::Cluster;
use crate::models::data_structure::job::Job;
use crate::models::utils::date_converter::format_timestamp;
use eframe::egui;

pub struct JobDetailsWindow {
    pub open: bool,
    pub job: Job,
    pub cluster: Vec<Cluster>,
}

impl JobDetailsWindow {
    pub fn new(job: Job, cluster: Vec<Cluster>) -> Self {
        Self {
            open: true,
            job: job,
            cluster: cluster,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // If the window is not open, do not render it
        if !self.open {
            return;
        }

        egui::Window::new(format!(
            "{}: {}",
            t!("app.details.general.title"),
            self.job.id
        ))
        .collapsible(true)
        .movable(true)
        .open(&mut self.open)
        .vscroll(true)
        .show(ui.ctx(), |ui| {
            // Base information
            ui.group(|ui| {
                ui.heading(t!("app.details.basic_info.title"));
                ui.horizontal(|ui| {
                    ui.label(format!("{}:", t!("app.details.basic_info.owner")));
                    ui.strong(self.job.owner.to_string());
                });
                ui.horizontal(|ui| {
                    ui.label(format!("{}:", t!("app.details.basic_info.queue")));
                    ui.strong(self.job.queue.to_string());
                });
                ui.horizontal(|ui| {
                    ui.label(format!("{}:", t!("app.details.basic_info.command")));
                    ui.strong(self.job.command.to_string());
                });
            });

            ui.add_space(8.0);

            // Status
            ui.group(|ui| {
                ui.heading(t!("app.details.status.title"));
                ui.horizontal(|ui| {
                    ui.label(format!("{}:", t!("app.details.status.state")));
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
                        ui.label(format!("{}:", t!("app.details.status.message")));
                        ui.label(message);
                    });
                }
            });

            ui.add_space(8.0);

            // Timing information
            ui.group(|ui| {
                ui.heading(t!("app.details.timing_info.title"));

                ui.horizontal(|ui| {
                    ui.label(format!(
                        "{}:",
                        t!("app.details.timing_info.submission_time")
                    ));
                    ui.strong(format_timestamp(self.job.submission_time));
                });
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "{}:",
                        t!("app.details.timing_info.scheduled_start_time")
                    ));
                    ui.strong(format_timestamp(self.job.scheduled_start));
                });
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "{}:",
                        t!("app.details.timing_info.actual_start_time")
                    ));
                    ui.strong(format_timestamp(self.job.start_time));
                });
                ui.horizontal(|ui| {
                    ui.label(format!("{}:", t!("app.details.timing_info.stop_time")));
                    ui.strong(format_timestamp(self.job.stop_time));
                });
                ui.horizontal(|ui| {
                    ui.label(format!("{}:", t!("app.details.timing_info.wall_time")));
                    ui.strong(format!("{} seconds", self.job.walltime));
                });
            });

            ui.add_space(8.0);

            if !self.cluster.is_empty() {
                // Ressources
                ui.group(|ui| {
                    ui.heading(t!("app.details.resources.title"));
                    egui::CollapsingHeader::new(t!("app.details.resources.cluster"))
                        .default_open(false)
                        .show(ui, |ui| {
                            for cluster in &self.cluster {
                                egui::CollapsingHeader::new(format!("{}", cluster.name))
                                    .default_open(false)
                                    .show(ui, |ui| {
                                        for host in &cluster.hosts {
                                            egui::CollapsingHeader::new(format!("{}", host.name))
                                                .default_open(false)
                                                .show(ui, |ui| {
                                                    ui.horizontal(|ui| {
                                                        ui.label("Network Address: ");
                                                        ui.strong(format!(
                                                            "{}",
                                                            host.network_address
                                                        ));
                                                    });

                                                    for cpu in &host.cpus {
                                                        egui::CollapsingHeader::new(format!(
                                                            "{}",
                                                            cpu.name
                                                        ))
                                                        .default_open(false)
                                                        .show(ui, |ui| {
                                                            ui.horizontal(|ui| {
                                                                ui.label("Chassis: ");
                                                                ui.strong(format!(
                                                                    "{}",
                                                                    cpu.chassis
                                                                ));
                                                            });

                                                            ui.horizontal(|ui| {
                                                                ui.label("Core Count: ");
                                                                ui.strong(format!(
                                                                    "{}",
                                                                    cpu.core_count
                                                                ));
                                                            });

                                                            ui.horizontal(|ui| {
                                                                ui.label("CPU Frequency: ");
                                                                ui.strong(format!(
                                                                    "{}",
                                                                    cpu.cpufreq
                                                                ));
                                                            });

                                                            for resource in &cpu.resources {
                                                                egui::CollapsingHeader::new(
                                                                    format!("{}", resource.id),
                                                                )
                                                                .default_open(false)
                                                                .show(ui, |ui| {
                                                                    ui.horizontal(|ui| {
                                                                        ui.label("State: ");
                                                                        ui.strong(format!(
                                                                            "{}",
                                                                            resource.state
                                                                        ));
                                                                    });
                                                                    ui.horizontal(|ui| {
                                                                        ui.label("Thread Count: ");
                                                                        ui.strong(format!(
                                                                            "{}",
                                                                            resource.thread_count
                                                                        ));
                                                                    });
                                                                });
                                                            }
                                                        });
                                                    }
                                                });
                                        }
                                    });
                            }
                        });
                });
            }

            ui.add_space(8.0);
        });
    }

    pub fn is_open(&self) -> bool {
        self.open
    }
}
