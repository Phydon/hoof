// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod gui;
use crate::gui::GuiMenu;
pub mod utils;

use eframe::egui;
use flexi_logger::{detailed_format, Duplicate, FileSpec, Logger};
use log::error;
use utils::check_create_config_dir;

use std::process;

const WINDOW_WIDTH: f32 = 380.0;
const WINDOW_HEIGHT: f32 = 560.0;

fn main() {
    // get config dir
    let config_dir = check_create_config_dir().unwrap_or_else(|err| {
        error!("Unable to find or create a config directory: {err}");
        process::exit(1);
    });

    // initialize the logger
    let _logger = Logger::try_with_str("info") // log warn and error
        .unwrap()
        .format_for_files(detailed_format) // use timestamp for every log
        .log_to_file(
            FileSpec::default()
                .directory(&config_dir)
                .suppress_timestamp(),
        ) // change directory for logs, no timestamps in the filename
        .append() // use only one logfile
        .duplicate_to_stderr(Duplicate::Info) // print infos, warnings and errors also to the console
        .start()
        .unwrap();

    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some(egui::Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT));
    eframe::run_native(
        "HOmeOFfice",
        options,
        Box::new(|_cc| Box::new(GuiMenu::default())),
    );
}
