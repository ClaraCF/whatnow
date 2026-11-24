use clap::{Parser, Subcommand};
use chrono::NaiveDate;

/// Yet another simple checklists tool
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Manage checklists
    Checklist {
        #[command(subcommand)]
        subcommand: ChecklistSubcommand,
    },

    /// Manage individual tasks
    Task {
        #[command(subcommand)]
        subcommand: TaskSubcommand
    }
}

#[derive(Subcommand)]
enum ChecklistSubcommand {
    /// Create a checklist
    Create {
        name: String,
    },

    /// Print all available checklists and their IDs
    List,

    /// Rename a checklist
    Rename {
        id: u32,
        new_name: String,
    },

    /// Delete a checklist
    Delete {
        id: u32,
    }
}

#[derive(Subcommand)]
enum TaskSubcommand {
    /// Add a task to a specific checklist
    Add {
        /// Checklist 
        checklist: Option<String>,
        description: String,
        priority: Option<i32>,
        deadline: Option<NaiveDate>,
    },

    /// List tasks in a specific checklist
    List {
        checklist: Option<String>,
        print_id: Option<bool>,
        // TODO: Add filter options
    },

    /// Edit an existing task
    Edit {
        id: u32,
        description: Option<String>,
        priority: Option<i32>,
        deadline: Option<NaiveDate>,
    },

    /// Move tasks from one checklist to another
    Move {
        id: u32,
        new_checklist: u32,
    },

    /// Mark a task as complete
    #[command(long_about = "Mark a task as complete. Does nothing if the task is already cheked")]
    Check {
        id: u32,
    },

    /// Mark a task as incomplete
    #[command(long_about = "Mark a task as complete. Does nothing if the task is already unchecked")]
    Uncheck {
        id: u32,
    },

    /// Toggle a task between complete and incomplete
    Toggle {
        id: u32,
    },

    /// Delete a task
    Delete {
        id: u32,
    },
}

struct Checklist {
    id: u32,
    name: String,
}

struct Task {
    id: u32,
    description: String,
    priority: Option<i32>,
    deadline: Option<NaiveDate>
}


fn parse_date(s: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%d-%m-%Y")
        .map_err(|e| format!("expected DD-MM-YYYY ({e})"))
}
