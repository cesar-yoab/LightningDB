use dashmap::DashMap;
use std::sync::Mutex;

pub struct DB {
    strings: Mutex<DashMap<String, String>>,
}

impl DB {
    pub fn new() -> DB {
        DB {
            strings: Mutex::new(DashMap::new()),
        }
    }

    pub fn strings_get(&self, key: &str) -> Result<String, &'static str> {
        match self.strings.lock().unwrap().get(key) {
            Some(value) => Ok(value.to_string()),
            None => Err("Value not found"),
        }
    }

    pub fn strings_set(&self, key: &str, value: &str) -> Option<String> {
        self.strings
            .lock()
            .unwrap()
            .insert(key.to_string(), value.to_string())
    }

    pub fn strings_del(&self, key: &str) -> Option<(String, String)> {
        self.strings.lock().unwrap().remove(key)
    }
}
