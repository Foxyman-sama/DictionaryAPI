use std::fmt::format;

use serde::{Deserialize, Serialize};

pub fn word_handler(word: &str) -> String {
    // generate_successful_json(translations)
    String::new()
}

#[derive(Debug, Serialize, Deserialize)]
struct Translation<'a> {
    part_of_speech: &'a str,
    sphere: &'a str,
    description: &'a str,
}

fn generate_successful_json(translations: Vec<Translation>) -> String {
    let mut result = String::new();
    let json = serde_json::to_string(&translations).unwrap();

    format!("{{\"status\":true,\"translations\":{}}}", json)
}

#[cfg(test)]
mod handler_tests {
    use super::*;

    const TEST_JSON: &'static str = r#"{"status":true,"translations":[{"part_of_speech":"дієслово","sphere":"загальна","description":"отримати"}]}"#;

    #[test]
    fn check_generation_of_successful_json_string() {
        let translations = vec![Translation {
            part_of_speech: "дієслово",
            sphere: "загальна",
            description: "отримати",
        }];

        let actual = generate_successful_json(translations);

        assert_eq!(TEST_JSON, actual)
    }
}
