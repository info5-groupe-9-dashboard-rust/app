#[derive(PartialEq)]
pub enum JobColorEnum {
    Random,
    State,
}

pub struct JobColor {
    pub color: JobColorEnum
}

impl Default for JobColor {
    fn default() -> Self {
        Self {
            color: JobColorEnum::Random
        }
    }
}

impl JobColor {

    pub fn is_random(&self) -> bool {
        self.color == JobColorEnum::Random
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Job color:");
            ui.horizontal(|ui|{
                ui.radio_value(&mut self.color, JobColorEnum::Random, "Random");
                ui.radio_value(&mut self.color, JobColorEnum::State, "State");
            });
        });
    }
}