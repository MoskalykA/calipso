use std::fs;
use serde::{Deserialize, Serialize};
use crate::calypso::{Calypso, initSaveFile, getFilePath};
#[derive(Debug, Deserialize, Serialize)]
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

fn addIdeaData(name: String, description: String, id: Option<i8>) {
   let fileContent = fs::read_to_string(getFilePath()).unwrap();
   let mut toJson: Calypso = serde_json::from_str(fileContent.as_str()).unwrap();

   if id.is_none() {
      let defaultIdea = Idea::default();
      let lastIdea = toJson.ideas.last().unwrap_or(&defaultIdea);
      toJson.ideas.push(Idea { id: lastIdea.id + 1, name: name, description: description });
   } else {
      toJson.ideas.push(Idea { id: id.unwrap(), name: name, description: description });
   }

   fs::write(getFilePath(), serde_json::to_string(&toJson).unwrap()).unwrap();
}

fn deleteIdeaData(id: i8) {
   let fileContent = fs::read_to_string(getFilePath()).unwrap();
   let mut toJson: Calypso = serde_json::from_str(fileContent.as_str()).unwrap();

   if let Some(idea) = toJson.ideas.iter().position(|x| x.id == id) {
      toJson.ideas.remove(idea);
   }

   fs::write(getFilePath(), serde_json::to_string(&toJson).unwrap()).unwrap();
}

fn updateIdeaData(id: i8, name: String, description: String) {
   initSaveFile();
   deleteIdeaData(id);
   addIdeaData(name, description, Some(id));
}

fn getIdeasData() -> Vec<Idea> {
   let fileContent = fs::read_to_string(getFilePath()).unwrap();
   let toJson: Calypso = serde_json::from_str(fileContent.as_str()).unwrap();
   toJson.ideas
}

fn findIdeaById(id: i8) -> Idea {
   let fileContent = fs::read_to_string(getFilePath()).unwrap();
   let mut toJson: Calypso = serde_json::from_str(fileContent.as_str()).unwrap();
   for idea in toJson.ideas {
      if idea.id == id {
         return idea
      }
   }

   Idea::default()
}

#[tauri::command]
pub fn add_idea_data(name: String, description: String) -> Vec<Idea> {
   initSaveFile();
   addIdeaData(name, description, None);
   getIdeasData()
}

#[tauri::command]
pub fn update_idea_data(id: i8, name: String, description: String) -> Vec<Idea> {
   initSaveFile();
   updateIdeaData(id, name, description);
   getIdeasData()
}

#[tauri::command]
pub fn request_ideas_data() -> Vec<Idea> {
   initSaveFile();
   getIdeasData()
}

#[tauri::command]
pub fn request_idea_data_by_id(id: i8) -> Idea {
   initSaveFile();
   findIdeaById(id)
}

#[tauri::command]
pub fn delete_idea_data(id: i8) -> Vec<Idea> {
   initSaveFile();
   deleteIdeaData(id);
   getIdeasData()
}