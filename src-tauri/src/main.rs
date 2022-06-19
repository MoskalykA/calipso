#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![request_knowledges_data, request_knowledges_data_with_id, delete_knowledges_data, add_knowledges, update_knowledges])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use tauri::Window;

#[derive(Debug, Deserialize, Serialize)]
struct Knowledges {
  id: i8,
  name: String,
  image: String,
  link: String
}

impl Default for Knowledges {
  fn default() -> Self {
    Knowledges { id: -1, name: "".to_string(), image: "".to_string(), link: "".to_string() }
  }
}

#[derive(Debug, Deserialize, Serialize)]
struct Calypso {
  knowledges: Vec<Knowledges>
}

fn init_knowledges_data() {
  if !Path::new("app.json").exists() {
    let data_base = Calypso {
      knowledges: Vec::new()
    };
    fs::write("app.json", serde_json::to_string(&data_base).unwrap()).unwrap();
  }
}

fn add_knowledges_to_data(name: String, image: String, link: String, id: Option<i8>) {
  if !Path::new("app.json").exists() {
    init_knowledges_data();
  }

  let file_content = fs::read_to_string("app.json").unwrap();
  let mut to_json: Calypso = serde_json::from_str(file_content.as_str()).unwrap();

  if id.is_none() {
    let default_knowledges = Knowledges::default();
    let knowledges = to_json.knowledges.last().unwrap_or(&default_knowledges);
    to_json.knowledges.push(Knowledges { id: knowledges.id + 1, name: name, image: image, link: link });
  } else {
    to_json.knowledges.push(Knowledges { id: id.unwrap(), name: name, image: image, link: link });
  }

  fs::write("app.json", serde_json::to_string(&to_json).unwrap()).unwrap();
}

fn delete_knowledges_to_data(id: i8) {
  if !Path::new("app.json").exists() {
    init_knowledges_data();
  }

  let file_content = fs::read_to_string("app.json").unwrap();
  let mut to_json: Calypso = serde_json::from_str(file_content.as_str()).unwrap();
  to_json.knowledges.remove((id - 1).try_into().unwrap());

  fs::write("app.json", serde_json::to_string(&to_json).unwrap()).unwrap();
}

fn update_knowledges_to_data(id: i8, name: String, image: String, link: String) {
  if !Path::new("app.json").exists() {
    init_knowledges_data();
  }

  delete_knowledges_to_data(id);
  add_knowledges_to_data(name, image, link, Some(id));
}

fn get_knowledges_data() -> Calypso {
  if !Path::new("app.json").exists() {
    init_knowledges_data();
  }

  let file_content = fs::read_to_string("app.json").unwrap();
  let to_json: Calypso = serde_json::from_str(file_content.as_str()).unwrap();
  to_json
}

fn find_knowledges_by_id(id: i8) -> Knowledges {
  if !Path::new("app.json").exists() {
    init_knowledges_data();
  }

  let file_content = fs::read_to_string("app.json").unwrap();
  let mut to_json: Calypso = serde_json::from_str(file_content.as_str()).unwrap();
  for knowledges in to_json.knowledges {
    if knowledges.id == id {
      return knowledges
    }
  }

  Knowledges::default()
}

fn send_knowledges_data(window: Window) {
  if !Path::new("app.json").exists() {
    init_knowledges_data();
  }
  
  let knowledges_data = get_knowledges_data();
  let from_json = serde_json::to_string(&knowledges_data).unwrap();
  window.emit("send_knowledges_data", from_json).unwrap();
}

#[tauri::command]
fn add_knowledges(window: Window, name: String, image: String, link: String) {
  add_knowledges_to_data(name, image, link, None);
  send_knowledges_data(window);
}

#[tauri::command]
fn update_knowledges(window: Window, id: i8, name: String, image: String, link: String) {
  update_knowledges_to_data(id, name, image, link);
  send_knowledges_data(window);
}

#[tauri::command]
fn request_knowledges_data(window: Window) {
  send_knowledges_data(window);
}

#[tauri::command]
fn request_knowledges_data_with_id(window: Window, id: i8) {
  if !Path::new("app.json").exists() {
    init_knowledges_data();
  }
  
  let knowledges_data =  find_knowledges_by_id(id);
  let from_json = serde_json::to_string(&knowledges_data).unwrap();
  window.emit("send_knowledges_data", from_json).unwrap();
}

#[tauri::command]
fn delete_knowledges_data(id: i8) {
  delete_knowledges_to_data(id);
}