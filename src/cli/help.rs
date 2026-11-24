// Root CLI
pub const CLI_ABOUT: &str = "Yet another simple checklists tool.";
pub const CLI_LONG_ABOUT: &str = "A simple tool to manage checklists, with support for multiple checklists, priorities and deadlines.";

// Checklist command
pub mod checklist {
    pub const ABOUT: &str = "Manage checklists";
    pub const LONG_ABOUT: &str = "Create, list, rename and delete checklists.";

    // Create subcommand
    pub mod create {
        pub const ABOUT: &str = "Create a new checklist";
        pub const LONG_ABOUT: &str = "Create a new checklist. Name is case sensitive.\nFails if a checklist with that name already exists.";
    }

    // List subcommand
    pub mod list {
        pub const ABOUT: &str = "Print all checklists";
        pub const LONG_ABOUT: &str = "Print all checklists. A checklist's ID is shown with it.\nOutput is empty if there are no checklists.";
        pub const AFTER_HELP: &str = "Examples:";   // TODO: Write an example
    }

    // Rename subcommand
    pub mod rename {
        pub const ABOUT: &str = "Rename a checklist";
        pub const LONG_ABOUT: &str = "Rename a checklist by its ID. New name is case sensitive.\nFails if another checklist with that name already exists or if the ID is invalid.";
        pub const AFTER_HELP: &str = "Examples:";   // TODO: Write an example
    }

    // Delete subcommand
    pub mod delete {
        pub const ABOUT: &str = "Delete a checklist";
        pub const LONG_ABOUT: &str = "Delete a checklist by its ID. Fails if the ID is invalid.";
        pub const AFTER_HELP: &str = "Examples:";   // TODO: Write an example
    }
}

// Task command
pub mod task {
    pub const ABOUT: &str = "Manage individual tasks";
    pub const LONG_ABOUT: &str = "Create, list, update, move or check/uncheck tasks.";

    // Add subcommand
    pub mod add {
        pub const ABOUT: &str = "Add a new task";
        pub const LONG_ABOUT: &str = "Add a new task to a specific checklist, with an optional priority and deadline.";
        pub const AFTER_HELP: &str = "Examples:";   // TODO: Write an example
    }

    // List subcommand
    pub mod list {
        pub const ABOUT: &str = "Print all tasks";
        pub const LONG_ABOUT: &str = "Print all tasks in a specific checklist. Output is empty if there are no tasks in the checklist.";
        pub const AFTER_HELP: &str = "Examples:";   // TODO: Write an example
    }

    // Edit subcommand
    pub mod edit {
        pub const ABOUT: &str = "Edit a task";
        pub const LONG_ABOUT: &str = "Edit the description, priotity or deadline of a task by its ID.\nFails if the ID is invalid.";
        pub const AFTER_HELP: &str = "Examples:";   // TODO: Write an example
    }

    // Move subcommand
    pub mod move_ {
        pub const ABOUT: &str = "Move a task to another checklist";
        pub const LONG_ABOUT: &str = "Move a task from one checklist to another, using the ID of the task and the target checklist.\nFails if the task ID or the target checklist are invalid.";
        pub const AFTER_HELP: &str = "Examples:";   // TODO: Write an example
    }

    // Check subcommand
    pub mod check {
        pub const ABOUT: &str = "Mark a task as complete";
        pub const LONG_ABOUT: &str = "Marks a task as complete. Does nothing if it is already marked as complete.";
        pub const AFTER_HELP: &str = "Examples:";   // TODO: Write an example
    }

    // Uncheck subcommand
    pub mod uncheck {
        pub const ABOUT: &str = "Mark a task as incomplete";
        pub const LONG_ABOUT: &str = "Marks a task as incomplete. Does nothing if it is already marked as incomplete.";
        pub const AFTER_HELP: &str = "Examples:";   // TODO: Write an example
    }

    // Toggle subcommand
    pub mod toggle {
        pub const ABOUT: &str = "Toggle a task as complete/incomplete";
        pub const LONG_ABOUT: &str = "Marks a task as complete if it is unchecked. Marks a task as incomplete if it is checked.";
        pub const AFTER_HELP: &str = "Examples:";   // TODO: Write an example
    }

    // Delete subcommand
    pub mod delete {
        pub const ABOUT: &str = "Delete a task";
        pub const LONG_ABOUT: &str = "Delete a task by its ID. Fails if the ID is invalid.";
        pub const AFTER_HELP: &str = "Examples:";   // TODO: Write an example
    }
}

