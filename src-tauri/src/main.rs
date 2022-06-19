#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod calypso;
mod knowledges;

use crate::knowledges::{add_knowledges, update_knowledges, request_knowledges_data, request_knowledges_data_with_id, delete_knowledges_data};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      add_knowledges, 
      update_knowledges, 
      request_knowledges_data, 
      request_knowledges_data_with_id, 
      delete_knowledges_data
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}