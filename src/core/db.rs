use std::collections::HashMap;
#[derive(Debug,Clone)]
pub struct DB {
    db: HashMap<String, String>,

    
}
impl DB {
    pub fn new() -> Self {
        DB { db: HashMap::new() }
    }

    
    pub fn clone(&self) -> Self {
        DB {
            db: self.db.clone(),
        }
    }
    pub fn set(&mut self, key: String, value: String) {
        self.db.insert(key, value);
    }

    pub fn get(&self, key: &String) -> Option<&String> {
        self.db.get(key)
    }

    pub fn remove(&mut self, key: &String) {
        self.db.remove(key);
    }

    pub fn exists(&self, key: &String) -> bool {
        self.db.contains_key(key)
    }

    pub fn keys(&self) -> Vec<String> {
        self.db.keys().cloned().collect()
    }
    pub fn flushall(&mut self) {
        self.db.clear();
    }
    
}
