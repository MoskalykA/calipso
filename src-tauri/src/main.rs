#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod calypso;
mod knowledges;
mod ideas;
mod settings;

#[macro_use]
extern crate lazy_static;

use crate::knowledges::{add_knowledge, update_knowledge, request_knowledges, request_knowledge_by_id, delete_knowledge, request_knowledge_count};
use crate::ideas::{add_idea, update_idea, request_ideas, request_idea_by_id, delete_idea, request_idea_count};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      add_knowledge, 
      update_knowledge, 
      request_knowledges, 
      request_knowledge_by_id, 
      delete_knowledge,
      request_knowledge_count,

      add_idea, 
      update_idea, 
      request_ideas, 
      request_idea_by_id, 
      delete_idea,
      request_idea_count
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}