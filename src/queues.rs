use once_cell::sync::Lazy;
use std::collections::{HashMap, VecDeque};
use tokio::sync::Mutex;

#[allow(clippy::type_complexity)]
pub static QUEUES: Lazy<Mutex<HashMap<String, (String, VecDeque<String>)>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
