use clap::{Parser, Subcommand};
use chrono::NaiveDate;

use crate::domain::checklist::ChecklistId;
use crate::domain::task::TaskId;

use super::help;

#[derive(Parser)]
#[command(version, about = help::CLI_ABOUT, long_about = help::CLI_LONG_ABOUT)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = help::checklist::ABOUT, long_about = help::checklist::LONG_ABOUT)]
    Checklist {
        #[command(subcommand)]
        subcommand: ChecklistSubcommand,
    },

    #[command(about = help::task::ABOUT, long_about = help::task::LONG_ABOUT)]
    Task {
        #[command(subcommand)]
        subcommand: TaskSubcommand
    }
}

#[derive(Subcommand)]
#[command(about = help::checklist::ABOUT, long_about = help::checklist::LONG_ABOUT)]
enum ChecklistSubcommand {
    #[command(
        about = help::checklist::create::ABOUT,
        long_about = help::checklist::create::LONG_ABOUT,
        after_help = help::checklist::create::AFTER_HELP,
    )]
    Create {
        name: String,
    },

    #[command(
        about = help::checklist::list::ABOUT,
        long_about = help::checklist::list::LONG_ABOUT,
        after_help = help::checklist::list::AFTER_HELP,
    )]
    List,

    #[command(
        about = help::checklist::rename::ABOUT,
        long_about = help::checklist::rename::LONG_ABOUT,
        after_help = help::checklist::rename::AFTER_HELP,
    )]
    Rename {
        id: ChecklistId,
        new_name: String,
    },

    #[command(
        about = help::checklist::delete::ABOUT,
        long_about = help::checklist::delete::LONG_ABOUT,
        after_help = help::checklist::delete::AFTER_HELP,
    )]
    Delete {
        id: ChecklistId,
    }
}

#[derive(Subcommand)]
#[command(about = help::task::ABOUT, long_about = help::task::LONG_ABOUT)]
enum TaskSubcommand {
    #[command(
        about = help::task::add::ABOUT,
        long_about = help::task::add::LONG_ABOUT,
        after_help = help::task::add::AFTER_HELP,
    )]
    Add {
        description: String,
        checklist: Option<ChecklistId>,
        priority: Option<i32>,
        deadline: Option<NaiveDate>,
    },

    #[command(
        about = help::task::list::ABOUT,
        long_about = help::task::list::LONG_ABOUT,
        after_help = help::task::list::AFTER_HELP,
    )]
    List {
        checklist: Option<ChecklistId>,
        //TODO: print_id: Option<bool>,
        // TODO: Add filter options
    },

    #[command(
        about = help::task::edit::ABOUT,
        long_about = help::task::edit::LONG_ABOUT,
        after_help = help::task::edit::AFTER_HELP,
    )]
    Edit {
        id: TaskId,
        description: Option<String>,
        priority: Option<i32>,
        deadline: Option<NaiveDate>,
    },

    #[command(
        about = help::task::move_::ABOUT,
        long_about = help::task::move_::LONG_ABOUT,
        after_help = help::task::move_::AFTER_HELP,
    )]
    Move {
        id: TaskId,
        new_checklist: ChecklistId,
    },

    #[command(
        about = help::task::check::ABOUT,
        long_about = help::task::check::LONG_ABOUT,
        after_help = help::task::check::AFTER_HELP,
    )]
    Check {
        id: TaskId,
    },

    #[command(
        about = help::task::uncheck::ABOUT,
        long_about = help::task::uncheck::LONG_ABOUT,
        after_help = help::task::uncheck::AFTER_HELP,
    )]
    Uncheck {
        id: TaskId,
    },

    #[command(
        about = help::task::toggle::ABOUT,
        long_about = help::task::toggle::LONG_ABOUT,
        after_help = help::task::toggle::AFTER_HELP,
    )]
    Toggle {
        id: TaskId,
    },

    #[command(
        about = help::task::delete::ABOUT,
        long_about = help::task::delete::LONG_ABOUT,
        after_help = help::task::delete::AFTER_HELP,
    )]
    Delete {
        id: TaskId,
    },
}

fn parse_date(s: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%d-%m-%Y")
        .map_err(|e| format!("expected DD-MM-YYYY ({e})"))
}
