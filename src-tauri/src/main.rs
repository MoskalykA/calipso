#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod calypso;
mod knowledges;
mod ideas;

#[macro_use]
extern crate lazy_static;

use crate::knowledges::{add_knowledge_data, update_knowledge_data, request_knowledges_data, request_knowledge_data_by_id, delete_knowledge_data};
use crate::ideas::{add_idea_data, update_idea_data, request_ideas_data, request_idea_data_by_id, delete_idea_data};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      add_knowledge_data, 
      update_knowledge_data, 
      request_knowledges_data, 
      request_knowledge_data_by_id, 
      delete_knowledge_data,

      add_idea_data, 
      update_idea_data, 
      request_ideas_data, 
      request_idea_data_by_id, 
      delete_idea_data
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}