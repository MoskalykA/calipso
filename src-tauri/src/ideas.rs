use std::fs;
use serde::{Deserialize, Serialize};
use crate::calypso::{Calypso, init_save_file, get_file_path};
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Idea {
   pub id: i8,
   pub name: String,
   pub description: String
}

impl Default for Idea {
   fn default() -> Self {
      Idea { id: -1, name: "".to_string(), description: "".to_string() }
   }
}

fn add_idea_data(name: String, description: String, id: Option<i8>) {
   let file_content = fs::read_to_string(get_file_path()).unwrap();
   let mut to_json: Calypso = serde_json::from_str(file_content.as_str()).unwrap();

   if id.is_none() {
      let default_idea = Idea::default();
      let last_idea = to_json.ideas.last().cloned().unwrap_or(default_idea);
      to_json.ideas.push(Idea { id: last_idea.id + 1, name: name, description: description });
   } else {
      to_json.ideas.push(Idea { id: id.unwrap(), name: name, description: description });
   }

   fs::write(get_file_path(), serde_json::to_string(&to_json).unwrap()).unwrap();
}

fn delete_idea_data(id: i8) {
   let file_content = fs::read_to_string(get_file_path()).unwrap();
   let mut to_json: Calypso = serde_json::from_str(file_content.as_str()).unwrap();

   if let Some(idea) = to_json.ideas.iter().position(|x| x.id == id) {
      to_json.ideas.remove(idea);
   }

   fs::write(get_file_path(), serde_json::to_string(&to_json).unwrap()).unwrap();
}

fn update_idea_data(id: i8, name: String, description: String) {
   init_save_file();
   delete_idea_data(id);
   add_idea_data(name, description, Some(id));
}

fn get_ideas_data() -> Vec<Idea> {
   let file_content = fs::read_to_string(get_file_path()).unwrap();
   let to_json: Calypso = serde_json::from_str(file_content.as_str()).unwrap();
   to_json.ideas
}

fn find_idea_by_id(id: i8) -> Idea {
   let file_content = fs::read_to_string(get_file_path()).unwrap();
   let to_json: Calypso = serde_json::from_str(file_content.as_str()).unwrap();
   for idea in to_json.ideas {
      if idea.id == id {
         return idea
      }
   }

   Idea::default()
}

#[tauri::command]
pub fn add_idea(name: String, description: String) -> Vec<Idea> {
   init_save_file();
   add_idea_data(name, description, None);
   get_ideas_data()
}

#[tauri::command]
pub fn update_idea(id: i8, name: String, description: String) -> Vec<Idea> {
   init_save_file();
   update_idea_data(id, name, description);
   get_ideas_data()
}

#[tauri::command]
pub fn request_ideas() -> Vec<Idea> {
   init_save_file();
   get_ideas_data()
}

#[tauri::command]
pub fn request_idea_by_id(id: i8) -> Idea {
   init_save_file();
   find_idea_by_id(id)
}

#[tauri::command]
pub fn delete_idea(id: i8) -> Vec<Idea> {
   init_save_file();
   delete_idea_data(id);
   get_ideas_data()
}