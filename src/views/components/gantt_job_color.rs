#[derive(PartialEq)]
pub enum JobColorEnum {
    Random,
    State,
}

pub struct JobColor {
    pub color: JobColorEnum,
}

impl Default for JobColor {
    fn default() -> Self {
        Self {
            color: JobColorEnum::Random, // Default value for the job color is Random
        }
    }
}

impl JobColor {

    /**
     * Returns true if the the selected job color is random
     */
    pub fn is_random(&self) -> bool {
        self.color == JobColorEnum::Random
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(format!("{}:", t!("app.gantt.settings.job_color")));
            // The job color can be random or based on the job state
            ui.horizontal(|ui| {
                ui.radio_value(
                    &mut self.color,
                    JobColorEnum::Random,
                    t!("app.gantt.settings.job_color_random"),
                );
                ui.radio_value(
                    &mut self.color,
                    JobColorEnum::State,
                    t!("app.gantt.settings.job_color_state"),
                );
            });
        });
    }
}
