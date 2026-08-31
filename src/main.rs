#![windows_subsystem = "windows"]

mod adb;
mod app;
mod config;
mod decode;

fn main() -> eframe::Result {
    let icon = load_icon();
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([450.0, 340.0])
            .with_title("Android Scan Simulator")
            .with_resizable(false)
            .with_icon(icon),
        ..Default::default()
    };
    eframe::run_native(
        "Android Scan Simulator",
        options,
        Box::new(|_cc| Ok(Box::new(app::ScanApp::default()))),
    )
}

fn load_icon() -> eframe::egui::IconData {
    let bytes = include_bytes!("../assets/icon.png");
    match image::load_from_memory(bytes) {
        Ok(img) => {
            let rgba = img.to_rgba8();
            let (w, h) = rgba.dimensions();
            eframe::egui::IconData {
                rgba: rgba.into_raw(),
                width: w,
                height: h,
            }
        }
        Err(_) => eframe::egui::IconData::default(),
    }
}
