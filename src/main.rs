mod app;
mod models;
mod views;

#[macro_use]
extern crate rust_i18n;
i18n!("src/i18n");

fn main() -> Result<(), eframe::Error> {
    // Configurez les options natives
    let options = eframe::NativeOptions::default();

    // Lancez l'application avec eframe
    eframe::run_native(
        &t!("app.title"),
        options,
        Box::new(|_cc| Ok(Box::new(app::App::new()))),
    )
}
