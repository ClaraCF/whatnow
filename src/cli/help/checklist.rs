// Checklist command
pub const ABOUT: &str = "Manage checklists";
pub const LONG_ABOUT: &str = "Create, list, rename and delete checklists.";

// Create subcommand
pub mod create {
    pub const ABOUT: &str = "Create a new checklist";
    pub const LONG_ABOUT: &str = "Create a new checklist. Name is case sensitive.\nFails if a checklist with that name already exists.";
    pub const AFTER_HELP: &str = "Examples:"; // TODO: Write an example
}

// List subcommand
pub mod list {
    pub const ABOUT: &str = "Print all checklists";
    pub const LONG_ABOUT: &str = "Print all checklists. A checklist's ID is shown with it.\nOutput is empty if there are no checklists.";
    pub const AFTER_HELP: &str = "Examples:"; // TODO: Write an example
}

// Rename subcommand
pub mod rename {
    pub const ABOUT: &str = "Rename a checklist";
    pub const LONG_ABOUT: &str = "Rename a checklist by its ID. New name is case sensitive.\nFails if another checklist with that name already exists or if the ID is invalid.";
    pub const AFTER_HELP: &str = "Examples:"; // TODO: Write an example
}

// Delete subcommand
pub mod delete {
    pub const ABOUT: &str = "Delete a checklist";
    pub const LONG_ABOUT: &str = "Delete a checklist by its ID. Fails if the ID is invalid.";
    pub const AFTER_HELP: &str = "Examples:"; // TODO: Write an example
}
