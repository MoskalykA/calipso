use std::fs;
use serde::{Deserialize, Serialize};
use tauri::Window;
use crate::calypso::{Calypso, initSaveFile};

#[derive(Debug, Deserialize, Serialize)]
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

fn addKnowledgeData(name: String, image: String, link: String, id: Option<i8>) {
   let fileContent = fs::read_to_string("app.json").unwrap();
   let mut toJson: Calypso = serde_json::from_str(fileContent.as_str()).unwrap();

   if id.is_none() {
      let defaultKnowledge = Knowledge::default();
      let lastKnowledge = toJson.knowledges.last().unwrap_or(&defaultKnowledge);
      toJson.knowledges.push(Knowledge { id: lastKnowledge.id + 1, name: name, image: image, link: link });
   } else {
      toJson.knowledges.push(Knowledge { id: id.unwrap(), name: name, image: image, link: link });
   }

   fs::write("app.json", serde_json::to_string(&toJson).unwrap()).unwrap();
}

fn deleteKnowledgeData(id: i8) {
   let fileContent = fs::read_to_string("app.json").unwrap();
   let mut toJson: Calypso = serde_json::from_str(fileContent.as_str()).unwrap();
   toJson.knowledges.remove((id - 1).try_into().unwrap());

   fs::write("app.json", serde_json::to_string(&toJson).unwrap()).unwrap();
}

fn updateKnowledgeData(id: i8, name: String, image: String, link: String) {
   initSaveFile();
   deleteKnowledgeData(id);
   addKnowledgeData(name, image, link, Some(id));
}

fn getKnowledgesData() -> Vec<Knowledge> {
   let fileContent = fs::read_to_string("app.json").unwrap();
   let toJson: Calypso = serde_json::from_str(fileContent.as_str()).unwrap();
   toJson.knowledges
}

fn findKnowledgeById(id: i8) -> Knowledge {
   let fileContent = fs::read_to_string("app.json").unwrap();
   let mut toJson: Calypso = serde_json::from_str(fileContent.as_str()).unwrap();
   for knowledge in toJson.knowledges {
      if knowledge.id == id {
         return knowledge
      }
   }

   Knowledge::default()
}

fn sendKnowledges(window: Window) {
   let knowledgesData = getKnowledgesData();
   let fromJson = serde_json::to_string(&knowledgesData).unwrap();
   window.emit("sendKnowledges", fromJson).unwrap();
}

fn sendKnowledge(window: Window, knowledge: Knowledge) {
   let fromJson = serde_json::to_string(&knowledge).unwrap();
   window.emit("sendKnowledge", fromJson).unwrap();
}

#[tauri::command]
pub fn add_knowledge_data(window: Window, name: String, image: String, link: String) {
   initSaveFile();
   addKnowledgeData(name, image, link, None);
   sendKnowledges(window);
}

#[tauri::command]
pub fn update_knowledge_data(window: Window, id: i8, name: String, image: String, link: String) {
   initSaveFile();
   updateKnowledgeData(id, name, image, link);
   sendKnowledges(window);
}

#[tauri::command]
pub fn request_knowledges_data(window: Window) {
   initSaveFile();
   sendKnowledges(window);
}

#[tauri::command]
pub fn request_knowledge_data_by_id(window: Window, id: i8) {
   initSaveFile();
   
   let knowledgeData = findKnowledgeById(id);
   sendKnowledge(window, knowledgeData);
}

#[tauri::command]
pub fn delete_knowledge_data(id: i8) {
   initSaveFile();
   deleteKnowledgeData(id);
}