use eframe::egui::{self, CentralPanel, TopBottomPanel};
use crate::models::application_context::ApplicationContext;
use crate::views::view::View;
use crate::views::menu::Menu;
use crate::views::dashboard::Dashboard;
use crate::views::gantt::GanttChart;

pub struct App {
    pub dashboard_view: Dashboard,
    pub gantt_view: GanttChart,
    pub menu : Menu,
    pub application_context: ApplicationContext,
}

impl App {
    pub fn new() -> Self {
        App {
            dashboard_view: Dashboard::default(),
            gantt_view: GanttChart::default(),
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

        CentralPanel::default().show(ctx, |ui| {
            match self.application_context.view_type {
                crate::views::view::ViewType::Dashboard => {
                    self.dashboard_view.render(ui, &mut self.application_context);
                }
                crate::views::view::ViewType::Gantt => {
                    self.gantt_view.render(ui, &mut self.application_context);
                }
            }
        });
    }

}