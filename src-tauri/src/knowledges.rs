use crate::{AppData, Payload};
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::Window;

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct Knowledge {
    pub name: String,
    pub image: String,
    pub link: String,
}

#[tauri::command]
pub fn add_knowledge(
    app_data: tauri::State<'_, AppData>,
    name: String,
    image: String,
    link: String,
) -> Vec<Knowledge> {
    let save_file = app_data.save_file.lock().unwrap().clone();
    let mut calypso = app_data.calypso.lock().unwrap();
    calypso.knowledges.push(Knowledge { name, image, link });

    fs::write(save_file, serde_json::to_string(&*calypso).unwrap()).unwrap();

    calypso.knowledges.clone()
}

#[tauri::command]
pub fn update_knowledge(
    window: Window,
    app_data: tauri::State<'_, AppData>,
    index: usize,
    name: String,
    image: String,
    link: String,
) -> Vec<Knowledge> {
    let mut calypso = app_data.calypso.lock().unwrap();
    match calypso.knowledges.get_mut(index) {
        Some(knowledge) => {
            knowledge.name = name;
            knowledge.image = image;
            knowledge.link = link;

            let save_file = app_data.save_file.lock().unwrap().clone();
            fs::write(save_file, serde_json::to_string(&*calypso).unwrap()).unwrap();
        }
        None => {
            window
                .emit(
                    "sendError",
                    Payload {
                        error: String::from(
                            "This knowledge does not exist, re-launch the application",
                        ),
                    },
                )
                .unwrap();
        }
    }

    calypso.knowledges.clone()
}

#[tauri::command]
pub fn request_knowledges(app_data: tauri::State<'_, AppData>) -> Vec<Knowledge> {
    let calypso = app_data.calypso.lock().unwrap().clone();
    calypso.knowledges
}

#[tauri::command]
pub fn request_knowledge_count(app_data: tauri::State<'_, AppData>) -> usize {
    let calypso = app_data.calypso.lock().unwrap().clone();
    calypso.knowledges.len()
}

#[tauri::command]
pub fn request_knowledge_by_id(
    window: Window,
    app_data: tauri::State<'_, AppData>,
    index: usize,
) -> Knowledge {
    let calypso = app_data.calypso.lock().unwrap().clone();
    match calypso.knowledges.get(index) {
        Some(knowledge) => knowledge.clone(),
        None => {
            window
                .emit(
                    "sendError",
                    Payload {
                        error: String::from(
                            "This knowledge does not exist, re-launch the application",
                        ),
                    },
                )
                .unwrap();

            Knowledge::default()
        }
    }
}

#[tauri::command]
pub fn delete_knowledge(
    window: Window,
    app_data: tauri::State<'_, AppData>,
    index: usize,
) -> Vec<Knowledge> {
    let mut calypso = app_data.calypso.lock().unwrap();
    match calypso.knowledges.get(index) {
        Some(_) => {
            calypso.knowledges.remove(index);

            let save_file = app_data.save_file.lock().unwrap().clone();
            fs::write(save_file, serde_json::to_string(&*calypso).unwrap()).unwrap();
        }
        None => {
            window
                .emit(
                    "sendError",
                    Payload {
                        error: String::from(
                            "This knowledge does not exist, re-launch the application",
                        ),
                    },
                )
                .unwrap();
        }
    };

    calypso.knowledges.clone()
}
