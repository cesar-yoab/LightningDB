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

    pub fn strings_append(&self, key: &str, append: &str) -> Option<String> {
        match self.strings_get(key) {
            Ok(old_value) => {
                let new_value = old_value + &append.to_string();
                return self.strings_set(key, new_value.as_str());
            }

            Err(_) => {
                return self.strings_set(key, append);
            }
        }
    }

    pub fn strings_len(&self, key: &str) -> Option<usize> {
        match self.strings_get(key) {
            Ok(value) => {
                return Some(value.len());
            }

            Err(_) => (),
        }

        None
    }
}
