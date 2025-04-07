use std::process::Command;

pub fn enable_raw_mode() {
    Command::new("stty")
        .args(["-icanon", "-echo", "-isig"])
        .status()
        .expect("Failed to enable partial raw mode");
}

pub fn disable_raw_mode() {
    Command::new("stty")
        .args(["icanon", "echo", "isig"])
        .status()
        .expect("Failed to restore terminal");
}
