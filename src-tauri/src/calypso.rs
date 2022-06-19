
use serde::{Deserialize, Serialize};
use crate::knowledges::Knowledges;

#[derive(Debug, Deserialize, Serialize)]
pub struct Calypso {
  pub knowledges: Vec<Knowledges>
}