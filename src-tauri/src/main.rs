// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod database;

use database::DatabaseManager;
use db_manager::User;
use once_cell::sync::Lazy;

static DBM: Lazy<DatabaseManager> = Lazy::new(|| DatabaseManager::connect().expect("Connection to database failed!"));

#[tauri::command]
fn validate_login(username: &str, password: &str) -> Option<User> {
    DBM.validate_login(username, password).expect("There was an issue with contacting the database!")
}

#[tauri::command]
fn add_user(new_user: User) {
    DBM.add_user(new_user).expect("There was an issue with adding a user!");
}

#[tauri::command]
fn debug_fetch_all() -> Vec<User> {
    DBM.get_users().find(None, None).unwrap().filter_map(|f| f.ok()).collect()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![validate_login, add_user, debug_fetch_all])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
