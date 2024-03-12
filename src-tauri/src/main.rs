// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! These are the main entry points for my frontend to reach the backend server.
//! Each of them has a procedural macro on them to transform the code that makes
//! them up at **compile time** so that they can interact with a WASM based frontend.

pub mod database;

use database::DatabaseManager;
use db_manager::{requests::UserReqError, User};
use once_cell::sync::Lazy;

// Statics are somewhat like constants - the important aspect of them that is relevant
// here is that they are available everywhere in the program, on every thread, no matter
// what. This means the connection to the DB is only initiallized once.
const DBM: Lazy<DatabaseManager> =
    Lazy::new(|| DatabaseManager::connect().expect("Connection to database failed!"));

/// This is one of the main commands that the frontend uses to communicate to the backend.
/// The reason why I need this is because the MongoDB driver I am using cannot run on a
/// WASM target, which is what the frontend is built upon. To solve this, Tauri - the library
/// which serves the frontend and creates the desktop window - provides me with a way to trigger
/// these functions from the WASM frontend, while actually executing them on the system process
/// in the backend.
///
/// This particular function runs my validate_login method on the DatabaseManager struct which
/// handles all logic related to: checking if the username and password are valid, checking if
/// the user is locked out, adding and resetting login attempts depending on whether or not the
/// login attempt was succesful, and of course, returning all of the final states of this process.
#[tauri::command]
fn validate_login(username: String, password: String) -> Result<User, UserReqError> {
    DBM.validate_login(username, password)
}

#[tauri::command]
fn add_user(new_user: User) -> Result<(), UserReqError> {
    DBM.add_user(new_user)
}

#[tauri::command]
fn debug_fetch_all() {
    // Debugging command to let me see the contents of the users table while
    // developing the program.
    println!(
        "{:#?}",
        DBM.get_users()
            .find(None, None)
            .unwrap()
            .collect::<Vec<Result<_, _>>>()
    );
}

#[tauri::command]
fn all_usernames() -> Vec<User> {
    // filter_map is a special iterator transformation function that applies a mutating function to
    // all elements of an iterator and filters out any None values from the resulting `Option<T>`
    // that is returned in each map pass.
    DBM.get_users()
        .find(None, None) // Find absolutely everything
        .unwrap() // Crash on fail
        .filter_map(|v| v.ok()) // Only keep correctly deserialized users.
        .collect() // Collect them into a Vec<T>
}

#[tauri::command]
fn unlock_user(username: String) -> Result<(), UserReqError> {
    DBM.unlock_user(username)
}

#[tauri::command]
fn delete_user(username: String) -> Result<(), UserReqError> {
    DBM.delete_user(username)
}

/// This is the starting point of the backend. It creates a `Builder` object
/// which is what I use to change the configuration of the program. It starts
/// off as a default, which is why I call the `std::default::Default` implementation
/// on it, but I also invoke the handlers that I generate for each of the commands
/// I am using on the frontend. Then the other configuration is loaded fro the
/// `tauri.conf.json` file inside the `src-tauri` workspace to load the other configuration
/// of the project.
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            validate_login,
            add_user,
            debug_fetch_all,
            all_usernames,
            unlock_user,
            delete_user
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
