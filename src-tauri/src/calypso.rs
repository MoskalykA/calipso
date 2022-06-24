use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::knowledges::Knowledge;
use crate::ideas::Idea;

lazy_static! {
   static ref DIR_PATH: String = if cfg!(dev) {
      "../data/".to_string()
   } else {
      "data".to_string()
   };

   static ref FILE_PATH: String = if cfg!(dev) {
      "../data/app.json".to_string()
   } else {
      "data/app.json".to_string()
   };
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Calypso {
  pub knowledges: Vec<Knowledge>,
  pub ideas: Vec <Idea>
}

pub fn get_file_path() -> String {
   FILE_PATH.clone()
}

fn get_dir_path() -> String {
   DIR_PATH.clone()
}

fn file_exists() -> bool {
   Path::new(&get_file_path()).exists()
}

fn dir_exists() -> bool {
   Path::new(&get_dir_path()).exists()
}

pub fn init_save_file() {
   if file_exists() {
      return
   }

   if !dir_exists() {
      fs::create_dir(get_dir_path()).unwrap();
   }

   let data_base = Calypso {
      knowledges: Vec::new(),
      ideas: Vec::new()
   };
   fs::write(FILE_PATH.clone(), serde_json::to_string(&data_base).unwrap()).unwrap();
}