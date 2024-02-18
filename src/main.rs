#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    eprintln!("Starting pollywog...");

    pollywog::start_app();
}
