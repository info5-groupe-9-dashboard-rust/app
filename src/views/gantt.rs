use crate::models::job::Job;
use eframe::egui;

pub fn render_gantt(ui: &mut egui::Ui) {
    let jobs = Job::get_jobs_from_json("src/data/jobs.json");
    
    ui.heading("Gantt Chart");

    let available_size = ui.available_size();
    let height = available_size.y;
    let width = available_size.x;

    let plot = egui_plot::Plot::new("gantt")
        .height(height)
        .width(width)
        .show_x(true)
        .show_y(true);

    plot.show(ui, |plot_ui| {
        for (i, job) in jobs.iter().enumerate() {
            let y = -(i as f64);
            let start = job.scheduled_start as f64;
            let duration = job.walltime as f64;
            
            let points = vec![
                [start, y],
                [start + duration, y],
            ];
            
            plot_ui.hline(egui_plot::HLine::new(y).width(10.0).color(egui::Color32::BLUE));
            plot_ui.line(egui_plot::Line::new(points).width(8.0).color(egui::Color32::BLUE));
        }
    });
}