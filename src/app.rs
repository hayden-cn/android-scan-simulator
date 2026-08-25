use eframe::egui;
use crate::adb;
use crate::config::AppConfig;

pub struct ScanApp {
    config: AppConfig,
    devices: Vec<String>,
    selected_device: usize,
    barcode: String,
    status: String,
}

impl Default for ScanApp {
    fn default() -> Self {
        let config = AppConfig::load();
        let devices = adb::list_devices();
        let selected_device = if !config.last_device.is_empty() {
            devices.iter().position(|d| d == &config.last_device).unwrap_or(0)
        } else {
            0
        };
        Self {
            config,
            devices,
            selected_device,
            barcode: String::new(),
            status: String::new(),
        }
    }
}

impl eframe::App for ScanApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Android Scan Simulator");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Device:");
                if self.devices.is_empty() {
                    ui.label("No devices found");
                } else {
                    egui::ComboBox::from_id_salt("device_select")
                        .selected_text(
                            self.devices
                                .get(self.selected_device)
                                .map(|s| s.as_str())
                                .unwrap_or(""),
                        )
                        .show_ui(ui, |ui| {
                            for (i, device) in self.devices.iter().enumerate() {
                                ui.selectable_value(&mut self.selected_device, i, device);
                            }
                        });
                }
                if ui.button("Refresh").clicked() {
                    self.devices = adb::list_devices();
                    if self.selected_device >= self.devices.len() {
                        self.selected_device = 0;
                    }
                }
            });

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label("Action:");
                ui.text_edit_singleline(&mut self.config.action);
            });

            ui.horizontal(|ui| {
                ui.label("Key:");
                ui.text_edit_singleline(&mut self.config.broadcast_key);
            });

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label("Barcode:");
                let response = ui.text_edit_singleline(&mut self.barcode);
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.send_scan();
                }
            });

            ui.add_space(8.0);

            if ui.button("Send Broadcast").clicked() {
                self.send_scan();
            }

            if !self.status.is_empty() {
                ui.separator();
                ui.label(&self.status);
            }
        });
    }
}

impl ScanApp {
    fn send_scan(&mut self) {
        if self.devices.is_empty() {
            self.status = "No device connected".to_string();
            return;
        }
        let device = &self.devices[self.selected_device];
        self.config.last_device = device.clone();
        let _ = self.config.save();

        match adb::send_broadcast(
            device,
            &self.config.action,
            &self.config.broadcast_key,
            &self.barcode,
        ) {
            Ok(output) => {
                self.status = format!("Sent! Response:\n{output}");
            }
            Err(e) => {
                self.status = format!("Error: {e}");
            }
        }
    }
}
