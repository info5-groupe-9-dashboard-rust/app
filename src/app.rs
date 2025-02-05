use eframe::egui::{self, CentralPanel, TopBottomPanel};
use crate::models::application_context::ApplicationContext;
use crate::views::view::View;
use crate::views::menu::Menu;
use crate::views::dashboard::Dashboard;
use crate::views::gantt::GanttChart;
use crate::views::options::Options;

pub struct App {
    pub dashboard_view: Dashboard,
    pub options_view: Options,
    pub gantt_view: GanttChart,
    pub menu : Menu,
    pub application_context: ApplicationContext,
}

impl App {
    pub fn new() -> Self {
        App {
            dashboard_view: Dashboard::default(),
            gantt_view: GanttChart::default(),
            options_view: Options::default(),
            menu: Menu::default(),
            application_context: ApplicationContext::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            self.menu.render(ui, &mut self.application_context);
        });

        // Vérifier les mises à jour
        self.application_context.check_jobs_update();

        CentralPanel::default().show(ctx, |ui| {
            // Afficher un indicateur de chargement si nécessaire
            if self.application_context.is_loading {
                ui.add_space(ui.available_height() * 0.4); // Push content down to vertical center
                ui.vertical_centered(|ui| {
                    ui.heading(t!("app.loading"));
                    // Optionnel : ajouter une animation de chargement
                    ui.spinner();
                });
                return;
            }
            
            match self.application_context.view_type {
                crate::views::view::ViewType::Dashboard => {
                    self.dashboard_view.render(ui, &mut self.application_context);
                }
                crate::views::view::ViewType::Gantt => {
                    self.gantt_view.render(ui, &mut self.application_context);
                }
                crate::views::view::ViewType::Options => {
                    self.options_view.render(ui, &mut self.application_context);
                }
            }
        });
    }

}