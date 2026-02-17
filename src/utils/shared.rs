use std::sync::{Arc, RwLock};

pub type Shared<T> = Arc<RwLock<T>>;

// constructor 
pub fn shared<T>(value: T) -> Shared<T> {
    Arc::new(RwLock::new(value))
}