use std::fs;
use serde::{Deserialize, Serialize};
use crate::calypso::{Calypso, init_save_file, get_file_path};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Knowledge {
   pub id: i8,
   pub name: String,
   pub image: String,
   pub link: String
}

impl Default for Knowledge {
   fn default() -> Self {
      Knowledge { id: -1, name: "".to_string(), image: "".to_string(), link: "".to_string() }
   }
}

fn add_knowledge_data(name: String, image: String, link: String, id: Option<i8>) {
   let file_content = fs::read_to_string(get_file_path()).unwrap();
   let mut to_json: Calypso = serde_json::from_str(file_content.as_str()).unwrap();

   if id.is_none() {
      let default_knowledge = Knowledge::default();
      let last_knowledge = to_json.knowledges.last().cloned().unwrap_or(default_knowledge);
      to_json.knowledges.push(Knowledge { id: last_knowledge.id + 1, name: name, image: image, link: link });
   } else {
      to_json.knowledges.push(Knowledge { id: id.unwrap(), name: name, image: image, link: link });
   }

   fs::write(get_file_path(), serde_json::to_string(&to_json).unwrap()).unwrap();
}

fn delete_knowledge_data(id: i8) {
   let file_content = fs::read_to_string(get_file_path()).unwrap();
   let mut to_json: Calypso = serde_json::from_str(file_content.as_str()).unwrap();

   if let Some(knowledge) = to_json.knowledges.iter().position(|x| x.id == id) {
      to_json.knowledges.remove(knowledge);
   }

   fs::write(get_file_path(), serde_json::to_string(&to_json).unwrap()).unwrap();
}

fn update_knowledge_data(id: i8, name: String, image: String, link: String) {
   init_save_file();
   delete_knowledge_data(id);
   add_knowledge_data(name, image, link, Some(id));
}

fn get_knowledges_data() -> Vec<Knowledge> {
   let file_content = fs::read_to_string(get_file_path()).unwrap();
   let to_json: Calypso = serde_json::from_str(file_content.as_str()).unwrap();
   to_json.knowledges
}

fn find_knowledge_by_id(id: i8) -> Knowledge {
   let file_content = fs::read_to_string(get_file_path()).unwrap();
   let to_json: Calypso = serde_json::from_str(file_content.as_str()).unwrap();
   for knowledge in to_json.knowledges {
      if knowledge.id == id {
         return knowledge
      }
   }

   Knowledge::default()
}

#[tauri::command]
pub fn add_knowledge(name: String, image: String, link: String) -> Vec<Knowledge> {
   init_save_file();
   add_knowledge_data(name, image, link, None);
   get_knowledges_data()
}

#[tauri::command]
pub fn update_knowledge(id: i8, name: String, image: String, link: String) -> Vec<Knowledge> {
   init_save_file();
   update_knowledge_data(id, name, image, link);
   get_knowledges_data()
}

#[tauri::command]
pub fn request_knowledges() -> Vec<Knowledge> {
   init_save_file();
   get_knowledges_data()
}

#[tauri::command]
pub fn request_knowledge_count() -> usize {
   init_save_file();
   get_knowledges_data().len()
}

#[tauri::command]
pub fn request_knowledge_by_id(id: i8) -> Knowledge {
   init_save_file();
   find_knowledge_by_id(id)
}

#[tauri::command]
pub fn delete_knowledge(id: i8) -> Vec<Knowledge> {
   init_save_file();
   delete_knowledge_data(id);
   get_knowledges_data()
}