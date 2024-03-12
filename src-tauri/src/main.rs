// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod infra;
mod models;
mod modules;
mod schema;
mod shared;
mod tests;
use serde::{Deserialize, Serialize};
use tauri::{Event, Manager};

use modules::user::entities::user::User;

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    any: String,
}

fn json_parse(json: &str) -> Payload {
    let payload: Payload = serde_json::from_str(&json).unwrap();
    payload
}

#[tauri::command]
fn ping(e: Event) {
    println!("{:?}", e);
    println!("pong!");

    let payload: Payload = json_parse(e.payload().unwrap());

    println!("{:?}", payload.any);
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            let win = app.get_window("main").unwrap();

            win.open_devtools();

            win.listen("ping", ping);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
