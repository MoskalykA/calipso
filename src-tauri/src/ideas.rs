use crate::{AppData, Payload};
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::Window;

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct Idea {
    pub name: String,
    pub description: String,
}

#[tauri::command]
pub fn add_idea(
    app_data: tauri::State<'_, AppData>,
    name: String,
    description: String,
) -> Vec<Idea> {
    let save_file = app_data.save_file.lock().unwrap().clone();
    let mut calypso = app_data.calypso.lock().unwrap();
    calypso.ideas.push(Idea { name, description });

    fs::write(save_file, serde_json::to_string(&*calypso).unwrap()).unwrap();

    calypso.ideas.clone()
}

#[tauri::command]
pub fn update_idea(
    window: Window,
    app_data: tauri::State<'_, AppData>,
    index: usize,
    name: String,
    description: String,
) -> Vec<Idea> {
    let mut calypso = app_data.calypso.lock().unwrap();
    match calypso.ideas.get_mut(index) {
        Some(idea) => {
            idea.name = name;
            idea.description = description;

            let save_file = app_data.save_file.lock().unwrap().clone();
            fs::write(save_file, serde_json::to_string(&*calypso).unwrap()).unwrap();
        }
        None => {
            window
                .emit(
                    "sendError",
                    Payload {
                        error: String::from("This idea does not exist, re-launch the application"),
                    },
                )
                .unwrap();
        }
    }

    calypso.ideas.clone()
}

#[tauri::command]
pub fn request_ideas(app_data: tauri::State<'_, AppData>) -> Vec<Idea> {
    let calypso = app_data.calypso.lock().unwrap().clone();
    calypso.ideas
}

#[tauri::command]
pub fn request_idea_count(app_data: tauri::State<'_, AppData>) -> usize {
    let calypso = app_data.calypso.lock().unwrap().clone();
    calypso.ideas.len()
}

#[tauri::command]
pub fn request_idea_by_id(
    window: Window,
    app_data: tauri::State<'_, AppData>,
    index: usize,
) -> Idea {
    let calypso = app_data.calypso.lock().unwrap().clone();
    match calypso.ideas.get(index) {
        Some(idea) => idea.clone(),
        None => {
            window
                .emit(
                    "sendError",
                    Payload {
                        error: String::from("This idea does not exist, re-launch the application"),
                    },
                )
                .unwrap();

            Idea::default()
        }
    }
}

#[tauri::command]
pub fn delete_idea(window: Window, app_data: tauri::State<'_, AppData>, index: usize) -> Vec<Idea> {
    let mut calypso = app_data.calypso.lock().unwrap();
    match calypso.ideas.get(index) {
        Some(_) => {
            calypso.ideas.remove(index);

            let save_file = app_data.save_file.lock().unwrap().clone();
            fs::write(save_file, serde_json::to_string(&*calypso).unwrap()).unwrap();
        }
        None => {
            window
                .emit(
                    "sendError",
                    Payload {
                        error: String::from("This idea does not exist, re-launch the application"),
                    },
                )
                .unwrap();
        }
    };

    calypso.ideas.clone()
}
