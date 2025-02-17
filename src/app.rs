use crate::views::menu::tools::Tools;
use crate::{models::data_structure::application_context::ApplicationContext, views::main_page::anthentification::Authentification};
use crate::views::main_page::dashboard::Dashboard;
use crate::views::main_page::gantt::GanttChart;
use crate::views::menu::menu::Menu;
use crate::views::view::View;
use eframe::egui::{self, CentralPanel, TopBottomPanel};

pub struct App {
    pub dashboard_view: Dashboard,
    pub gantt_view: GanttChart,
    pub authentification_view: Authentification,
    pub menu: Menu,
    pub tools: Tools,
    pub application_context: ApplicationContext
}

impl App {
    pub fn new() -> Self {

        let app = App {
            dashboard_view: Dashboard::default(),
            gantt_view: GanttChart::default(),
            authentification_view: Authentification::default(),
            menu: Menu::default(),
            tools: Tools::default(),
            application_context: ApplicationContext::default(),
        };

        app
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            self.menu.render(ui, &mut self.application_context);
        });

        // Check for updates
        self.application_context.check_data_update();

        CentralPanel::default().show(ctx, |ui| {
            
            TopBottomPanel::top("tool_bar").show(ctx, |ui| {
                self.tools.render(ui, &mut self.application_context);
            });

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
                    crate::views::view::ViewType::Authentification => {
                        self.authentification_view.render(ui, &mut self.application_context);
                    }
                }
            });
        });
    }
}
