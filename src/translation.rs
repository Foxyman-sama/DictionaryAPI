use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Translation {
  pub part_of_speech: String,
  pub sphere: String,
  pub description: String,
}
