use serde::{Deserialize, Serialize};
use super::task::Task;

pub type ChecklistId = u32;

#[derive(Serialize, Deserialize)]
pub struct Checklist {
    id: ChecklistId,
    name: String,
    tasks: Vec<Task>,
}

impl Checklist {
    pub fn new(id: ChecklistId, name: &str) -> Checklist {
        Checklist {
            id,
            name: String::from(name),
            tasks: vec![],
        }
    }
}
