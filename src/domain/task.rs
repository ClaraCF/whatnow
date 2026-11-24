use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

pub type TaskId = u32;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    id: TaskId,
    description: String,
    priority: u32,
    deadline: Option<NaiveDate>
}

impl Task {
    pub fn new(id: TaskId, description: &str, priority: Option<u32>, deadline: Option<NaiveDate>) -> Task {
        Task {
            id,
            description: String::from(description),
            priority: priority.unwrap_or(0),
            deadline,
        }
    }
}




