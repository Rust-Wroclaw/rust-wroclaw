use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub enum Value {
    Empty,
    String(String),
    Int(i32),
}

pub fn hash_value(v: &Value) -> u64 {
    let mut hasher = DefaultHasher::new();

    match v {
        Value::Empty => ().hash(&mut hasher),
        Value::String(x) => x.hash(&mut hasher),
        Value::Int(x) => x.hash(&mut hasher),
    }

    hasher.finish()
}
