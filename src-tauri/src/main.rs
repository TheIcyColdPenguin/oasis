#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod data_types;
mod database;

use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    static ref DATABASE_MANAGER: Arc<Mutex<database::DatabaseManager>> = Arc::new(Mutex::new(database::DatabaseManager::new().unwrap()));
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::get_persons])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
