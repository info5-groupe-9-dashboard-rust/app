use crate::views::View;
use crate::app::App;

pub struct Menu;

impl Menu {
    pub fn render(ui: &mut egui::Ui, app: &mut App) {
        ui.horizontal(|ui| {
            ui.menu_button("Fichier", |ui| {
                if ui.button("Quitter").clicked() {
                    std::process::exit(0);
                }
            });
            ui.menu_button("Vue", |ui| {
                if ui.button("Tableau de bord").clicked() {
                    app.current_view = View::Dashboard;
                }
                if ui.button("Diagramme de Gantt").clicked() {
                    app.current_view = View::Gantt;
                }
            });
        });
    }
}