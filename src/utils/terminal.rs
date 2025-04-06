use std::process::Command;

pub fn enable_raw_mode() {
    Command::new("styy")
        .arg("-echo")
        .arg("raw")
        .status()
        .expect("Failed to enter raw mode");
}

pub fn disable_raw_mode() {
    Command::new("stty")
        .arg("echo")
        .arg("-raw")
        .status()
        .expect("Failed to exit raw mode");
}
