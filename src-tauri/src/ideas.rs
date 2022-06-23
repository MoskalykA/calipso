use std::fs;
use serde::{Deserialize, Serialize};
use tauri::Window;
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

fn sendIdeas(window: Window) {
   let ideasData = getIdeasData();
   let fromJson = serde_json::to_string(&ideasData).unwrap();
   window.emit("sendIdeas", fromJson).unwrap();
}

fn sendIdea(window: Window, idea: Idea) {
   let fromJson = serde_json::to_string(&idea).unwrap();
   window.emit("sendIdea", fromJson).unwrap();
}

#[tauri::command]
pub fn add_idea_data(window: Window, name: String, description: String) {
   initSaveFile();
   addIdeaData(name, description, None);
   sendIdeas(window);
}

#[tauri::command]
pub fn update_idea_data(window: Window, id: i8, name: String, description: String) {
   initSaveFile();
   updateIdeaData(id, name, description);
   sendIdeas(window);
}

#[tauri::command]
pub fn request_ideas_data(window: Window) {
   initSaveFile();
   sendIdeas(window);
}

#[tauri::command]
pub fn request_idea_data_by_id(window: Window, id: i8) {
   initSaveFile();
   
   let ideaData = findIdeaById(id);
   sendIdea(window, ideaData);
}

#[tauri::command]
pub fn delete_idea_data(window: Window, id: i8) {
   initSaveFile();
   deleteIdeaData(id);
   sendIdeas(window);
}