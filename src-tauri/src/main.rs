// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ideas;
mod knowledges;

use ideas::Idea;
use knowledges::Knowledge;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use crate::ideas::{
    add_idea, delete_idea, request_idea_by_id, request_idea_count, request_ideas, update_idea,
};
use crate::knowledges::{
    add_knowledge, delete_knowledge, request_knowledge_by_id, request_knowledge_count,
    request_knowledges, update_knowledge,
};

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct Calypso {
    pub knowledges: Vec<Knowledge>,
    pub ideas: Vec<Idea>,
}

#[derive(Deserialize, Serialize)]
pub struct AppData {
    pub calypso: Mutex<Calypso>,
    pub save_file: Mutex<String>,
}

#[derive(Clone, Serialize)]
struct Payload {
    error: String,
}

impl AppData {
    fn new(save_file: String) -> Self {
        Self {
            calypso: Mutex::new(Calypso::default()),
            save_file: Mutex::new(save_file),
        }
    }
}

fn main() {
    let save_file = String::from("data.json");
    let app_data = AppData::new(save_file.clone());
    if !std::path::Path::new(&save_file).exists() {
        std::fs::create_dir_all(std::path::Path::new(&save_file).parent().unwrap()).unwrap();

        std::fs::write(
            save_file,
            serde_json::to_string(&Calypso::default()).unwrap(),
        )
        .unwrap();
    }

    tauri::Builder::default()
        .manage(app_data)
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
