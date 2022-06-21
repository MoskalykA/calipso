use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::knowledges::Knowledge;
use crate::ideas::Idea;

lazy_static! {
   static ref DIR_PATH: String = "../data".to_string();
   static ref FILE_PATH: String = "../data/app.json".to_string();
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Calypso {
  pub knowledges: Vec<Knowledge>,
  pub ideas: Vec <Idea>
}

pub fn getFilePath() -> String {
   FILE_PATH.clone()
}

fn getDirPath() -> String {
   DIR_PATH.clone()
}

fn fileExists() -> bool {
   Path::new(&getFilePath()).exists()
}

fn dirExists() -> bool {
   Path::new(&getDirPath()).exists()
}

pub fn initSaveFile() {
   if fileExists() {
      return
   }

   if !dirExists() {
      fs::create_dir(getDirPath());
   }

   let dataBase = Calypso {
      knowledges: Vec::new(),
      ideas: Vec::new()
   };
   fs::write(FILE_PATH.clone(), serde_json::to_string(&dataBase).unwrap()).unwrap();
}