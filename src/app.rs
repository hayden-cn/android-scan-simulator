use eframe::egui;
use std::sync::mpsc;
use crate::adb;
use crate::config::AppConfig;

enum BroadcastResult {
    Ok(String),
    Err(String),
}

pub struct ScanApp {
    config: AppConfig,
    devices: Vec<String>,
    selected_device: usize,
    barcode: String,
    status: String,
    receiver: Option<mpsc::Receiver<BroadcastResult>>,
    selected_profile: usize,
    show_new_profile: bool,
    new_profile_name: String,
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
        let selected_profile = config.active_profile_index();
        Self {
            config,
            devices,
            selected_device,
            barcode: String::new(),
            status: String::new(),
            receiver: None,
            selected_profile,
            show_new_profile: false,
            new_profile_name: String::new(),
        }
    }
}

impl eframe::App for ScanApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(rx) = &self.receiver {
            if let Ok(result) = rx.try_recv() {
                match result {
                    BroadcastResult::Ok(output) => {
                        self.status = format!("Sent! Response:\n{output}");
                    }
                    BroadcastResult::Err(e) => {
                        self.status = format!("Error: {e}");
                    }
                }
                self.receiver = None;
            } else {
                ctx.request_repaint();
            }
        }

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
                ui.label("Profile:");
                egui::ComboBox::from_id_salt("profile_select")
                    .selected_text(
                        self.config
                            .profiles
                            .get(self.selected_profile)
                            .map(|p| p.name.as_str())
                            .unwrap_or(""),
                    )
                    .show_ui(ui, |ui| {
                        let profiles: Vec<(usize, String)> = self
                            .config
                            .profiles
                            .iter()
                            .enumerate()
                            .map(|(i, p)| (i, p.name.clone()))
                            .collect();
                        for (i, name) in profiles {
                            if ui
                                .selectable_label(self.selected_profile == i, &name)
                                .clicked()
                            {
                                self.selected_profile = i;
                                self.config.select_profile(i);
                                let _ = self.config.save();
                            }
                        }
                    });

                if ui.small_button("+").clicked() {
                    self.show_new_profile = true;
                    self.new_profile_name.clear();
                }

                if ui.small_button("-").clicked() && self.config.profiles.len() > 1 {
                    let idx = self.selected_profile;
                    self.config.delete_profile(idx);
                    self.selected_profile = self.config.active_profile_index();
                    let _ = self.config.save();
                }
            });

            if self.show_new_profile {
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    let name_response = ui.text_edit_singleline(&mut self.new_profile_name);
                    if ui.button("OK").clicked()
                        || (name_response.lost_focus()
                            && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                    {
                        let name = self.new_profile_name.trim().to_string();
                        if !name.is_empty()
                            && self.config.add_profile(
                                &name,
                                &self.config.action.clone(),
                                &self.config.broadcast_key.clone(),
                            )
                        {
                            self.selected_profile = self.config.active_profile_index();
                            let _ = self.config.save();
                            self.status = format!("Profile '{}' created", name);
                        } else if !name.is_empty() {
                            self.status = format!("Profile '{}' already exists", name);
                        }
                        self.show_new_profile = false;
                    }
                    if ui.button("Cancel").clicked() {
                        self.show_new_profile = false;
                    }
                });
            }

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label("Action:");
                let response = ui.text_edit_singleline(&mut self.config.action);
                if response.changed() {
                    let idx = self.selected_profile;
                    if let Some(profile) = self.config.profiles.get_mut(idx) {
                        profile.action = self.config.action.clone();
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label("Key:");
                let response = ui.text_edit_singleline(&mut self.config.broadcast_key);
                if response.changed() {
                    let idx = self.selected_profile;
                    if let Some(profile) = self.config.profiles.get_mut(idx) {
                        profile.key = self.config.broadcast_key.clone();
                    }
                }
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

            let sending = self.receiver.is_some();
            if ui.add_enabled(!sending, egui::Button::new("Send Broadcast")).clicked() {
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
        let device = self.devices[self.selected_device].clone();
        let action = self.config.action.clone();
        let key = self.config.broadcast_key.clone();
        let value = self.barcode.clone();

        self.config.last_device = device.clone();
        let _ = self.config.save();

        let (tx, rx) = mpsc::channel();
        self.receiver = Some(rx);
        self.status = "Sending...".to_string();

        std::thread::spawn(move || {
            let result = match adb::send_broadcast(&device, &action, &key, &value) {
                Ok(output) => BroadcastResult::Ok(output),
                Err(e) => BroadcastResult::Err(e),
            };
            let _ = tx.send(result);
        });
    }
}
