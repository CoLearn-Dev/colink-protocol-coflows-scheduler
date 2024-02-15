pub mod coflows_push;
pub mod coflows_scheduler;
pub mod init;
pub mod queues;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FlowTask {
    flow_id: String,
    message_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct FlowTasks {
    flow_id: String,
    message_ids: Vec<String>,
}
