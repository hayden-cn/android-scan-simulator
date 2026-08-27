#![windows_subsystem = "windows"]

mod adb;
mod app;
mod config;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([450.0, 340.0])
            .with_title("Android Scan Simulator")
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "Android Scan Simulator",
        options,
        Box::new(|_cc| Ok(Box::new(app::ScanApp::default()))),
    )
}
