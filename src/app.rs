use eframe::egui::{self, CentralPanel, TopBottomPanel};
use crate::models::application_context::ApplicationContext;
use crate::models::application_options::ApplicationOptions;
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
        let application_options = ApplicationOptions::default();
        let options_view = if std::path::Path::new("options.json").exists() {
            Options::load_from_file("options.json")
        } else {
            Options::new(application_options.clone())
        };

        let app = App {
            dashboard_view: Dashboard::default(),
            gantt_view: GanttChart::default(),
            options_view,
            menu: Menu::default(),
            application_context: ApplicationContext::default()
        };

        app
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // Apply application options
        self.options_view.apply_options(ctx);

        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            self.menu.render(ui, &mut self.application_context);
        });

        // Check for updates
        self.application_context.check_jobs_update();

        CentralPanel::default().show(ctx, |ui| {
            // Display a loading indicator if necessary
            if self.application_context.is_loading {
                ui.add_space(ui.available_height() * 0.4);
                ui.vertical_centered(|ui| {
                    ui.heading(t!("app.loading"));
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