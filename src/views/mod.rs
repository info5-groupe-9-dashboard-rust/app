pub mod dashboard;
pub mod gantt;
pub mod menu;

pub enum View {
    Dashboard,
    Gantt,
}

pub fn render_view(ui: &mut egui::Ui, view: &View) {
    match view {
        View::Dashboard => {
            dashboard::render_dashboard(ui);
        }
        View::Gantt => {
            gantt::render_gantt(ui);
        }
    }
}