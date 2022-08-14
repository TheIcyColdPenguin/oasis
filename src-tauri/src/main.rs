#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod data_types;
mod database;

use data_types::Person;
use tauri::command;

use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    static ref DATABASE_MANAGER: Arc<Mutex<database::DatabaseManager>> = Arc::new(Mutex::new(database::DatabaseManager::new().unwrap()));
}

#[command]
fn hello() -> Result<Vec<Person>, ()> {
    let lock = DATABASE_MANAGER.lock().unwrap();
    let result = lock.get_persons().unwrap();
    Ok(result)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![hello])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
