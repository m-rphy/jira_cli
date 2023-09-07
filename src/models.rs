use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed
}

pub struct Epic {
    name: String,
    description: String,
    status: Status,
    stories: Vec<i32>
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Epic {
            name,
            description,
            status: Status::Open,
            stories: vec![]
        }
    }
}

pub struct Story {
    name: String,
    description: String,
    status: Status
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Story {
            name,
            description,
            status: Status::Open,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DBState {
    // This struct represents the entire db state which includes the last_item_id, epics, and stories
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>
}
