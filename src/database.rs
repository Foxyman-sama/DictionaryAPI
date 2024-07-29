#[path = "translation.rs"]
mod translation;

use postgres::{Client, NoTls};
pub use translation::Translation;

const HOSTNAME: &'static str = "localhost";
const USERNAME: &'static str = "postgres";
const PASSWORD: &'static str = "root"; // it seems strange to set it like this, hahaha
const DBNAME: &'static str = "dictionary";

const DEFAULT_NULL_VALUE: &'static str = "null";
const PART_OF_SPEECH_INDEX: usize = 2;
const SPHERE_INDEX: usize = 3;
const DESCRIPTION_INDEX: usize = 4;

pub struct Database {
  client: Client,
}

impl Database {
  pub fn connect() -> Database {
    let args = format!(
      "host={} user={} password={} dbname={}",
      HOSTNAME, USERNAME, PASSWORD, DBNAME
    );
    Database {
      client: Client::connect(args.as_str(), NoTls).unwrap(),
    }
  }

  pub fn get_translations(&mut self, word: &str) -> Vec<Translation> {
    let rows = self.get(word);
    make_translations_from_rows(rows)
  }

  fn get(&mut self, word: &str) -> Vec<postgres::Row> {
    self
      .client
      .query("SELECT * FROM test WHERE word = $1", &[&word])
      .unwrap()
  }
}

fn make_translations_from_rows(rows: Vec<postgres::Row>) -> Vec<Translation> {
  rows.iter().map(|row| make_translation(&row)).collect()
}

fn make_translation(row: &postgres::Row) -> Translation {
  let part_of_speech = try_get(&row, PART_OF_SPEECH_INDEX);
  let sphere = try_get(&row, SPHERE_INDEX);
  let description = try_get(&row, DESCRIPTION_INDEX);

  Translation {
    part_of_speech,
    sphere,
    description,
  }
}

fn try_get(row: &postgres::Row, index: usize) -> String {
  row.try_get(index).unwrap_or(String::from(DEFAULT_NULL_VALUE))
}

#[cfg(test)]
mod database_tests {
  use super::*;

  const TEST_RECORD: &'static str = "NULL_TEST";

  fn generate_test_translation() -> Translation {
    Translation {
      part_of_speech: String::from(TEST_RECORD),
      sphere: String::from(TEST_RECORD),
      description: String::from(TEST_RECORD),
    }
  }

  #[test]
  fn can_get_value_from_db() {
    let mut db = Database::connect();

    let actual = db.get("ab-");
    let actual: &str = actual[0].get(1);

    assert_eq!("ab-", actual);
  }

  #[test]
  fn get_from_database_in_translation_type() {
    let expected = generate_test_translation();
    let mut db = Database::connect();

    let actual = db.get_translations(TEST_RECORD);
    let actual = &actual[0];

    assert_eq!(expected.part_of_speech, actual.part_of_speech);
    assert_eq!(expected.sphere, actual.sphere);
    assert_eq!(expected.description, actual.description);
  }
}
