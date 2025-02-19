use crate::models::data_structure::job::Job;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SortBy {
    Time,
    Owner,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Sorting {
    pub sort_by: SortBy,
    pub reversed: bool,
}

impl Default for Sorting {
    fn default() -> Self {
        Self {
            sort_by: SortBy::Time,
            reversed: false,
        }
    }
}

impl Sorting {
    pub fn sort(self, mut jobs: Vec<Job>) -> Vec<Job> {
        match self.sort_by {
            SortBy::Time => {
                jobs.sort_by_key(|info| info.start_time);
            }
            SortBy::Owner => {
                jobs.sort_by(|a, b| a.owner.cmp(&b.owner));
            }
        }
        if self.reversed {
            jobs.reverse();
        }
        jobs
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Sort jobs by:");

            let dir = if self.reversed { '⬆' } else { '⬇' };

            for &sort_by in &[SortBy::Time, SortBy::Owner] {
                let selected = self.sort_by == sort_by;

                let label = if selected {
                    format!("{sort_by:?} {dir}")
                } else {
                    format!("{sort_by:?}")
                };

                if ui.add(egui::RadioButton::new(selected, label)).clicked() {
                    if selected {
                        self.reversed = !self.reversed;
                    } else {
                        self.sort_by = sort_by;
                        self.reversed = false;
                    }
                }
            }
        });
    }
}