use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::knowledges::Knowledge;
use crate::ideas::Idea;

lazy_static! {
   static ref FILE_PATH: String = "app.json".to_string();
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Calypso {
  pub knowledges: Vec<Knowledge>,
  pub ideas: Vec <Idea>
}

pub fn saveFileExists() -> bool {
   Path::new(&FILE_PATH.clone()).exists()
}

pub fn initSaveFile() {
   if saveFileExists() {
      return
   }

   let dataBase = Calypso {
      knowledges: Vec::new(),
      ideas: Vec::new()
   };
   fs::write(FILE_PATH.clone(), serde_json::to_string(&dataBase).unwrap()).unwrap();
}