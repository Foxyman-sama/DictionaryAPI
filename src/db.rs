use std::fmt::format;

use postgres::{Client, NoTls};

pub struct Database {
  client: Client,
}

impl Database {
  pub fn new(args: &[&str]) -> Database {
    let args = format!(
      "host={} user={} password={} dbname={}",
      args[0], args[1], args[2], args[3]
    );
    Database {
      client: Client::connect(args.as_str(), NoTls).unwrap(),
    }
  }

  pub fn get(&mut self, word: &str) -> Vec<postgres::Row> {
    self
      .client
      .query("SELECT * FROM test WHERE word = $1", &[&word])
      .unwrap()
  }
}

#[cfg(test)]
mod database_tests {
  use super::*;

  #[test]
  fn can_get_value_from_db() {
    let mut database = Database::new(&["localhost", "postgres", "root", "dictionary"]);

    let actual = database.get("ab-");
    let actual: &str = actual[0].get(1);

    assert_eq!("ab-", actual);
  }
}
