//! Mock database implementation for testing without a real database

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use sled::{IVec, Result};
use mockall::mock;
use mockall::predicate::*;

use kosync::defs::ProgressState;

/// MockDB provides a memory-based implementation of the database for testing
#[derive(Clone)]
pub struct MockDB {
    users: Arc<Mutex<HashMap<String, String>>>,
    docs: Arc<Mutex<HashMap<String, Vec<u8>>>>,
}

impl MockDB {
    /// Create a new MockDB instance
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
            docs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Add a user to the mock database
    pub fn put_user(&self, name: &str, key: &str) -> Result<Option<IVec>> {
        let mut users = self.users.lock().unwrap();
        let old_value = users.insert(format!("U:{}:K", name), key.to_string());
        Ok(old_value.map(|v| IVec::from(v.as_bytes())))
    }

    /// Get a user from the mock database
    pub fn get_user(&self, name: &str) -> Result<Option<IVec>> {
        let users = self.users.lock().unwrap();
        let key = format!("U:{}:K", name);
        Ok(users.get(&key).map(|v| IVec::from(v.as_bytes())))
    }

    /// Add a document to the mock database
    pub fn put_doc(&self, user: &str, doc: &str, value: &ProgressState) -> Result<Option<IVec>> {
        let mut docs = self.docs.lock().unwrap();
        let key = format!("U:{}:D:{}", user, doc);
        
        match serde_json::to_vec(value) {
            Ok(v) => {
                let old_value = docs.insert(key, v.clone());
                Ok(old_value.map(IVec::from))
            },
            Err(_) => Ok(None),
        }
    }

    /// Get a document from the mock database
    pub fn get_doc(&self, user: &str, doc: &str) -> Result<Option<ProgressState>> {
        let docs = self.docs.lock().unwrap();
        let key = format!("U:{}:D:{}", user, doc);
        
        match docs.get(&key) {
            Some(v) => Ok(serde_json::from_slice(&v).ok()),
            None => Ok(None),
        }
    }
}

// Create a MockDB trait for testing with mockall
pub trait MockDBTrait {
    fn put_user(&self, name: &str, key: &str) -> Result<Option<IVec>>;
    fn get_user(&self, name: &str) -> Result<Option<IVec>>;
    fn put_doc(&self, user: &str, doc: &str, value: &ProgressState) -> Result<Option<IVec>>;
    fn get_doc(&self, user: &str, doc: &str) -> Result<Option<ProgressState>>;
}

// Implement the trait for our MockDB
impl MockDBTrait for MockDB {
    fn put_user(&self, name: &str, key: &str) -> Result<Option<IVec>> {
        self.put_user(name, key)
    }
    
    fn get_user(&self, name: &str) -> Result<Option<IVec>> {
        self.get_user(name)
    }
    
    fn put_doc(&self, user: &str, doc: &str, value: &ProgressState) -> Result<Option<IVec>> {
        self.put_doc(user, doc, value)
    }
    
    fn get_doc(&self, user: &str, doc: &str) -> Result<Option<ProgressState>> {
        self.get_doc(user, doc)
    }
}

// Create a mock implementation of the trait
mock! {
    pub MockDBImpl {}
    
    impl MockDBTrait for MockDBImpl {
        fn put_user(&self, name: &str, key: &str) -> Result<Option<IVec>>;
        fn get_user(&self, name: &str) -> Result<Option<IVec>>;
        fn put_doc(&self, user: &str, doc: &str, value: &ProgressState) -> Result<Option<IVec>>;
        fn get_doc(&self, user: &str, doc: &str) -> Result<Option<ProgressState>>;
    }
}