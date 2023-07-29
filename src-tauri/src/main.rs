// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn read() -> String {
    format!("Trying to read..")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, read])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    println!("Does this printout?");
}
