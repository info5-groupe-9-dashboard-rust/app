mod app;
mod models;
mod views;
mod controllers;

fn main() -> Result<(), eframe::Error> {
    // Configurez les options natives
    let options = eframe::NativeOptions::default();

    // Lancez l'application avec eframe
    eframe::run_native(
        "Dashboard HPC",
        options,
        Box::new(|_cc| Ok(Box::new(app::App::default()))),
    )
}
