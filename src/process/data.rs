use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Clone)]
pub struct ProcessData {
    pub pid: u32,
    name: String,
    uid: Option<u32>,
    pub username: Option<String>
}

impl ProcessData {
    pub fn new(pid: u32, name: String, uid: Option<u32>, username: Option<String>) -> Self {
        ProcessData { pid, name, uid, username }
    }
}

pub type ProcessList = Arc<Mutex<Vec<ProcessData>>>;
pub fn empty_process_list() -> ProcessList {
    Arc::new(Mutex::new(Vec::new()))
}

pub type ProcessStreams = Arc<Mutex<Vec<mpsc::UnboundedSender<ProcessData>>>>;


#[derive(Deserialize)]
pub struct SearchOptions {
    pub pid: Option<u32>,
    pub username: Option<String>
}
