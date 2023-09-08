use std::collections::HashMap;
use serde::{Serialize, Deserialize};



#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    NavigateToEpicDetail {epic_id: u32},
    NavigateToStoryDetail { epic_id: u32, story_id: u32 },
    NavigateToPreviousPage,
    CreateEpic,
    UpdateEpicStatus { epic_id: u32 },
    DeleteEpic { epic_id: u32 },
    CreateStory {epic_id: u32},
    UpdateStoryStatus { story_id: u32 },
    DeleteStory { epic_id: u32, story_id: u32 },
    Exit,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<i32>
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status
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

#[derive(Serialize, Deserialize, Clone)]
pub struct DBState {
    // This struct represents the entire db state which includes the last_item_id, epics, and stories
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>
}
