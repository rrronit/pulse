use std::{fmt::Error, sync::Arc};
use tokio::sync::RwLock;

use super::db::DB;

#[derive(Debug)]
pub struct Commands {
    commands: String,
    pub args: Vec<String>,
}

impl Commands {
    pub fn new(commands: String, args: Vec<String>) -> Commands {
        Commands { commands, args }
    }

    pub async fn handle_get_command(&self, db: Arc<RwLock<DB>>) -> Option<String> {
        // Acquire a read lock on the database
        let db = db.read().await;
        let value = db.get(&self.args[0]);

        match value {
            Some(value) => Some(value.to_string()),
            None => None,
        }
    }

    pub async fn handle_set_command(&self, db: Arc<RwLock<DB>>) {
        let mut db = db.write().await;
        db.set(self.args[0].clone(), self.args[1].clone());
    }

    pub async fn handle_del_command(&self, db: Arc<RwLock<DB>>) {
        let mut db = db.write().await;
        db.remove(&self.args[0]);
    }

    pub async fn handle_keys_command(&self, db: Arc<RwLock<DB>>) -> Vec<String> {
        let db = db.read().await;
        let pattern = &self.args[0];
        let keys = db.keys();
        let mut result = Vec::new();
        for key in keys {
            if key.contains(pattern) {
                result.push(key);
            }
        }
        result
    }
    pub async fn handle_exists_command(&self, db: Arc<RwLock<DB>>) -> i32 {
        let db = db.read().await;
        self.args
            .iter()
            .fold(0, |acc, key| if db.exists(key) { acc + 1 } else { acc })
    }

    pub fn handle_ping_command(&self) -> String {
        if self.args.is_empty() {
            "PONG".to_string()
        } else {
            self.args[0].clone()
        }
    }

    pub async fn handle_flushall_command(&self, db: Arc<RwLock<DB>>) {
        let mut db = db.write().await;
        db.flushall();

    }

    pub async fn handle_incr_command(&self, db: Arc<RwLock<DB>>) -> Result<i32, Error> {
        let mut db = db.write().await;
        let key = &self.args[0];
        let value_str = match db.get(key) {
            Some(value) => value.clone(),
            None => {
                db.set(key.clone(), "0".to_string());
                "0".to_string()
            }
        };

        let value = value_str.parse::<i32>();

        if let Ok(v) = value {
            db.set(key.clone(), (v + 1).to_string());
            Ok(v + 1)
        } else {
            Err(Error)
        }
    }
    pub async fn handle_decr_command(&self, db: Arc<RwLock<DB>>) -> Result<i32, Error> {
        let mut db = db.write().await;
        let key = &self.args[0];
        let value_str = match db.get(key) {
            Some(value) => value.clone(),
            None => {
                db.set(key.clone(), "0".to_string());
                "0".to_string()
            }
        };

        let value = value_str.parse::<i32>();

        if let Ok(v) = value {
            db.set(key.clone(), (v - 1).to_string());
            Ok(v - 1)
        } else {
            Err(Error)
        }
    }

}
