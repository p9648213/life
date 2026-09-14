use std::collections::HashMap;

pub struct Cookie {}

impl Cookie {
    pub fn parse(value: &str) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for pair in value.split(";") {
            if let Some((key, value)) = pair.split_once("=") {
                map.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
        map
    }
}
