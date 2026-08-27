use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

fn adb_command() -> Command {
    let mut cmd = Command::new("adb");
    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);
    cmd
}

pub fn list_devices() -> Vec<String> {
    let output = match adb_command().arg("devices").output() {
        Ok(o) => o,
        Err(_) => return Vec::new(),
    };
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .skip(1)
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let serial = parts.next()?;
            let state = parts.next()?;
            if state == "device" {
                Some(serial.to_string())
            } else {
                None
            }
        })
        .collect()
}

pub fn send_broadcast(device: &str, action: &str, key: &str, value: &str) -> Result<String, String> {
    let output = adb_command()
        .args([
            "-s", device,
            "shell", "am", "broadcast",
            "-a", action,
            "--es", key, value,
        ])
        .output()
        .map_err(|e| format!("Failed to run adb: {e}"))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
