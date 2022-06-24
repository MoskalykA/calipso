use std::fs;
use serde::{Deserialize, Serialize};
use crate::calypso::{Calypso, initSaveFile, getFilePath};

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
   let fileContent = fs::read_to_string(getFilePath()).unwrap();
   let mut toJson: Calypso = serde_json::from_str(fileContent.as_str()).unwrap();

   if id.is_none() {
      let defaultKnowledge = Knowledge::default();
      let lastKnowledge = toJson.knowledges.last().unwrap_or(&defaultKnowledge);
      toJson.knowledges.push(Knowledge { id: lastKnowledge.id + 1, name: name, image: image, link: link });
   } else {
      toJson.knowledges.push(Knowledge { id: id.unwrap(), name: name, image: image, link: link });
   }

   fs::write(getFilePath(), serde_json::to_string(&toJson).unwrap()).unwrap();
}

fn deleteKnowledgeData(id: i8) {
   let fileContent = fs::read_to_string(getFilePath()).unwrap();
   let mut toJson: Calypso = serde_json::from_str(fileContent.as_str()).unwrap();

   if let Some(knowledge) = toJson.knowledges.iter().position(|x| x.id == id) {
      toJson.knowledges.remove(knowledge);
   }

   fs::write(getFilePath(), serde_json::to_string(&toJson).unwrap()).unwrap();
}

fn updateKnowledgeData(id: i8, name: String, image: String, link: String) {
   initSaveFile();
   deleteKnowledgeData(id);
   addKnowledgeData(name, image, link, Some(id));
}

fn getKnowledgesData() -> Vec<Knowledge> {
   let fileContent = fs::read_to_string(getFilePath()).unwrap();
   let toJson: Calypso = serde_json::from_str(fileContent.as_str()).unwrap();
   toJson.knowledges
}

fn findKnowledgeById(id: i8) -> Knowledge {
   let fileContent = fs::read_to_string(getFilePath()).unwrap();
   let mut toJson: Calypso = serde_json::from_str(fileContent.as_str()).unwrap();
   for knowledge in toJson.knowledges {
      if knowledge.id == id {
         return knowledge
      }
   }

   Knowledge::default()
}

#[tauri::command]
pub fn add_knowledge_data(name: String, image: String, link: String) -> Vec<Knowledge> {
   initSaveFile();
   addKnowledgeData(name, image, link, None);
   getKnowledgesData()
}

#[tauri::command]
pub fn update_knowledge_data(id: i8, name: String, image: String, link: String) -> Vec<Knowledge> {
   initSaveFile();
   updateKnowledgeData(id, name, image, link);
   getKnowledgesData()
}

#[tauri::command]
pub fn request_knowledges_data() -> Vec<Knowledge> {
   initSaveFile();
   getKnowledgesData()
}

#[tauri::command]
pub fn request_knowledge_data_by_id(id: i8) -> Knowledge {
   initSaveFile();
   findKnowledgeById(id)
}

#[tauri::command]
pub fn delete_knowledge_data(id: i8) -> Vec<Knowledge> {
   initSaveFile();
   deleteKnowledgeData(id);
   getKnowledgesData()
}