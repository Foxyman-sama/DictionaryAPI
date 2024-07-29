#[path = "database.rs"]
mod database;

use database::Translation;

const ERROR_JSON: &'static str = r#"{"status":"not found"}"#;

pub fn make_responce_from_db(word: &str) -> String {
  let mut db = database::Database::connect();
  let translations = db.get_translations(word);

  if translations.len() > 0 {
    generate_successful_json(translations)
  } else {
    generate_error_json()
  }
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

  const TEST_SUCCESSFUL_RESPONCE: &'static str = r#"{"status":true,"translations":[{"part_of_speech":"null","sphere":"null","description":"словах,с общим значением удаления"}]}"#;

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
