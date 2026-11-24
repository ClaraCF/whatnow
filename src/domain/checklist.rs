use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

use super::task::Task;

pub type ChecklistId = u32;

#[derive(Serialize, Deserialize, Clone, Debug)]
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

    // Returns a checklist read from disk
    pub fn from_disk(path: PathBuf) -> Result<Checklist> {
        let raw = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => {
                return Err(e.into());
            }
        };

        match serde_json::from_str(&raw) {
            Ok(checklist) => Ok(checklist),
            Err(e) => Err(e.into()),
        }
    }

    // Writes the checklist to dir_path/name.json
    pub fn write_to_disk(&self, dir_path: &Path) -> Result<()> {
        let path = dir_path.join(format!("{}.json", self.name));

        let json =
            serde_json::to_string_pretty(self).context("failed to serialize tasks to JSON")?;

        fs::write(&path, json)
            .with_context(|| format!("failed to write checklist to {}", path.display()))?;

        Ok(())
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
}
