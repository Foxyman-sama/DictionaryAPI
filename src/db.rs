use postgres::{Client, NoTls};
use tide::http::headers::HOST;

const HOSTNAME: &'static str = "localhost";
const USERNAME: &'static str = "postgres";
const PASSWORD: &'static str = "root"; // it seems strange to set it like this, hahaha
const DBNAME: &'static str = "dictionary";

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
    let mut database = Database::connect();

    let actual = database.get("ab-");
    let actual: &str = actual[0].get(1);

    assert_eq!("ab-", actual);
  }
}
