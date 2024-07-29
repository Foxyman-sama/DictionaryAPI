use db::Database;
use serde::{Deserialize, Serialize};
#[path = "db.rs"]
mod db;

const DEFAULT_NULL_VALUE: &'static str = "null";
const PART_OF_SPEECH_INDEX: usize = 2;
const SPHERE_INDEX: usize = 3;
const DESCRIPTION_INDEX: usize = 4;

const ERROR_JSON: &'static str = r#"{"status":"not found"}"#;

#[derive(Debug, Serialize, Deserialize)]
struct Translation {
  part_of_speech: String,
  sphere: String,
  description: String,
}

pub fn make_responce_from_db(word: &str) -> String {
  let mut db = Database::connect();
  let rows = db.get(word);
  let translations = make_translations_from_rows(rows);

  if translations.len() > 0 {
    generate_successful_json(translations)
  } else {
    generate_error_json()
  }
}

fn make_translations_from_rows(rows: Vec<postgres::Row>) -> Vec<Translation> {
  let mut translations = Vec::new();

  for row in rows {
    let part_of_speech = try_get(&row, PART_OF_SPEECH_INDEX);
    let sphere = try_get(&row, SPHERE_INDEX);
    let description = try_get(&row, DESCRIPTION_INDEX);
    let transl = Translation {
      part_of_speech,
      sphere,
      description,
    };

    translations.push(transl)
  }

  translations
}

fn try_get(row: &postgres::Row, index: usize) -> String {
  row.try_get(index).unwrap_or(String::from(DEFAULT_NULL_VALUE))
}

fn generate_successful_json(translations: Vec<Translation>) -> String {
  let json = serde_json::to_string(&translations).unwrap();

  format!("{{\"status\":true,\"translations\":{}}}", json)
}

fn generate_error_json() -> String {
  String::from(ERROR_JSON)
}

#[cfg(test)]
mod handler_tests {
  use super::*;

  const TEST_JSON: &'static str =
    r#"{"status":true,"translations":[{"part_of_speech":"дієслово","sphere":"загальна","description":"отримати"}]}"#;

  const TEST_SUCCESSFUL_RESPONCE: &'static str = r#"{"status":true,"translations":[{"part_of_speech":"null","sphere":"null","description":"словах,с общим значением удаления"}]}"#;

  #[test]
  fn check_generation_of_successful_json_string() {
    let translations = vec![Translation {
      part_of_speech: String::from("дієслово"),
      sphere: String::from("загальна"),
      description: String::from("отримати"),
    }];

    let actual = generate_successful_json(translations);

    assert_eq!(TEST_JSON, actual)
  }

  #[test]
  fn check_generation_of_making_responce_fromb_db() {
    let actual = make_responce_from_db("ab-");

    assert_eq!(TEST_SUCCESSFUL_RESPONCE, actual);
  }

  #[test]
  fn return_error_responce_when_word_doesnt_exist() {
    let actual = make_responce_from_db("fjdsfidsfjsdfh");

    assert_eq!(ERROR_JSON, actual);
  }
}
