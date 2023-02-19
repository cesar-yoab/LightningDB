use std::collections::HashMap;

pub struct DB {
    kv_map: HashMap<String, String>,
}

impl DB {
    pub fn new() -> DB {
        DB {
            kv_map: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: &str) -> Result<String, &'static str> {
        match self.kv_map.get(key) {
            Some(value) => Ok(value.to_string()),
            None => Err("Value not found"),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> Option<String> {
        self.kv_map.insert(key.to_string(), value.to_string())
    }

    pub fn del(&mut self, key: &str) -> Option<String> {
        self.kv_map.remove(key)
    }
}
