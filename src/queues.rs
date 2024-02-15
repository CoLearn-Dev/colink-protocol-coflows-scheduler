use once_cell::sync::Lazy;
use std::collections::{HashMap, VecDeque};
use tokio::sync::Mutex;

pub static QUEUES: Lazy<Mutex<HashMap<String, VecDeque<String>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
