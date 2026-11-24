// Task command
pub const ABOUT: &str = "Manage individual tasks";
pub const LONG_ABOUT: &str = "Create, list, update, move or check/uncheck tasks.";

// Add subcommand
pub mod add {
    pub const ABOUT: &str = "Add a new task";
    pub const LONG_ABOUT: &str =
        "Add a new task to a specific checklist, with an optional priority and deadline.";
    pub const AFTER_HELP: &str = "Examples:"; // TODO: Write an example
}

// List subcommand
pub mod list {
    pub const ABOUT: &str = "Print all tasks";
    pub const LONG_ABOUT: &str = "Print all tasks in a specific checklist. Output is empty if there are no tasks in the checklist.";
    pub const AFTER_HELP: &str = "Examples:"; // TODO: Write an example
}

// Edit subcommand
pub mod edit {
    pub const ABOUT: &str = "Edit a task";
    pub const LONG_ABOUT: &str = "Edit the description, priority or deadline of a task by its ID.\nFails if the ID is invalid.";
    pub const AFTER_HELP: &str = "Examples:"; // TODO: Write an example
}

// Move subcommand
pub mod move_ {
    pub const ABOUT: &str = "Move a task to another checklist";
    pub const LONG_ABOUT: &str = "Move a task from one checklist to another, using the ID of the task and the target checklist.\nFails if the task ID or the target checklist are invalid.";
    pub const AFTER_HELP: &str = "Examples:"; // TODO: Write an example
}

// Check subcommand
pub mod check {
    pub const ABOUT: &str = "Mark a task as complete";
    pub const LONG_ABOUT: &str =
        "Marks a task as complete. Does nothing if it is already marked as complete.";
    pub const AFTER_HELP: &str = "Examples:"; // TODO: Write an example
}

// Uncheck subcommand
pub mod uncheck {
    pub const ABOUT: &str = "Mark a task as incomplete";
    pub const LONG_ABOUT: &str =
        "Marks a task as incomplete. Does nothing if it is already marked as incomplete.";
    pub const AFTER_HELP: &str = "Examples:"; // TODO: Write an example
}

// Toggle subcommand
pub mod toggle {
    pub const ABOUT: &str = "Toggle a task as complete/incomplete";
    pub const LONG_ABOUT: &str =
        "Marks a task as complete if it is unchecked. Marks a task as incomplete if it is checked.";
    pub const AFTER_HELP: &str = "Examples:"; // TODO: Write an example
}

// Delete subcommand
pub mod delete {
    pub const ABOUT: &str = "Delete a task";
    pub const LONG_ABOUT: &str = "Delete a task by its ID. Fails if the ID is invalid.";
    pub const AFTER_HELP: &str = "Examples:"; // TODO: Write an example
}
