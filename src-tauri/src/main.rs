#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::net::ToSocketAddrs;

use tcp_smart_socket::*;

fn main() {

    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[tauri::command]
fn turn_on() -> Result<(), ()> {
    Ok(())
}

#[tauri::command]
fn turn_off() -> Result<(), ()> {
    Ok(())
}

#[tauri::command]
fn status() -> Result<(), ()> {
    Ok(())
}

async fn connect_smart_socket(addr: impl ToSocketAddrs) -> tcp_smart_socket::C