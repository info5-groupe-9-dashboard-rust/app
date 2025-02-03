use eframe::egui::{self, CentralPanel, TopBottomPanel};
use crate::views::view::View;
use crate::views::menu::Menu;
use crate::views::dashboard::Dashboard;
use crate::views::gantt::GanttChart;
use crate::models::job::*;

pub struct App {
    pub current_view: Box<dyn View>,
    pub jobs: Vec<Job>
}

impl App {
    pub fn new() -> Self {
        let jobs = get_current_jobs();
        App {
            current_view: Box::new(Dashboard::new(jobs.clone())),
            jobs : jobs
        }
    }

    pub fn switch_to_gantt(&mut self) {
        self.current_view = Box::new(GanttChart::new(self.jobs.clone()));
    }

    pub fn switch_to_dashboard(&mut self) {
        self.current_view = Box::new(Dashboard::new(self.jobs.clone()));
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            Menu::render(ui, self);
        });

        CentralPanel::default().show(ctx, |ui| self.current_view.render(ui));
    }

}