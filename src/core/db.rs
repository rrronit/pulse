use std::{collections::HashMap, time};
#[derive(Debug, Clone)]
pub struct DB {
    db: HashMap<String, String>,
    expire_db: HashMap<String, time::Instant>,
}
impl DB {
    pub fn new() -> Self {
        DB {
            db: HashMap::new(),
            expire_db: HashMap::new(),
        }
    }

    pub fn clone(&self) -> Self {
        DB {
            db: self.db.clone(),
            expire_db: self.expire_db.clone(),
        }
    }
    pub fn set(&mut self, key: String, value: String, ex: Option<time::Duration>) {
        self.db.insert(key.clone(), value);
        if let Some(ex) = ex {
            self.expire_db.insert(key, time::Instant::now() + ex);
        }
    }

    pub fn get(&mut self, key: &String) -> Option<&String> {
        if let Some(expire_time) = self.expire_db.get(key) {
            if expire_time < &time::Instant::now() {
                self.remove(key);
                return None;
            }
        }
        self.db.get(key)
    }

    pub fn remove(&mut self, key: &String) {
        self.db.remove(key);
        self.expire_db.remove(key);
    }

    pub fn exists(&self, key: &String) -> bool {
        self.db.contains_key(key)
    }

    pub fn keys(&self) -> Vec<String> {
        self.db.keys().cloned().collect()
    }
    pub fn flushall(&mut self) {
        self.db.clear();
        self.expire_db.clear();
    }
}
