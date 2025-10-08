// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// This module re-exports all the main functionality from main.rs
pub mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // This function is kept for mobile compatibility but the actual app runs through main.rs
    println!("Use main.rs entry point for desktop applications");
}
