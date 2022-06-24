use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {

}

impl Default for Settings {
   fn default() -> Self {
      Settings {

      }
   }
}