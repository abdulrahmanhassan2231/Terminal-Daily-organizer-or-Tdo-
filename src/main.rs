use clap::{Parser, Subcommand};
use tdo::{add_task, list_tasks, complete_task, delete_task, export_tasks, clear_done};

#[derive(Parser)]
#[command(
    name = "tdo",
    about = "Terminal Daily Organizer — stay in flow, stay on task",
    version = "0.1.0"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        /// Task description
        description: Vec<String>,
        /// Priority: high, med, low
        #[arg(short, long, default_value = "med")]
        priority: String,
    },
    /// List all tasks
    List {
        /// Show only incomplete tasks
        #[arg(short, long)]
        pending: bool,
    },
    /// Mark a task as done
    Done {
        /// Task ID
        id: u32,
    },
    /// Delete a task
    Delete {
        /// Task ID
        id: u32,
    },
    /// Export tasks to a markdown file
    Export {
        #[arg(short, long, default_value = "tasks.md")]
        output: String,
    },
    /// Remove all completed tasks
    ClearDone,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { description, priority } => {
            let desc = description.join(" ");
            match add_task(&desc, &priority) {
                Ok(id) => println!("Task #{id} added: {desc}"),
                Err(e) => eprintln!("Error: {e}"),
            }
        }
        Commands::List { pending } => {
            if let Err(e) = list_tasks(pending) {
                eprintln!("Error: {e}");
            }
        }
        Commands::Done { id } => {
            match complete_task(id) {
                Ok(_) => println!("Task #{id} marked as done!"),
                Err(e) => eprintln!("Error: {e}"),
            }
        }
        Commands::Delete { id } => {
            match delete_task(id) {
                Ok(_) => println!("Task #{id} deleted."),
                Err(e) => eprintln!("Error: {e}"),
            }
        }
        Commands::Export { output } => {
            match export_tasks(&output) {
                Ok(_) => println!("Tasks exported to {output}"),
                Err(e) => eprintln!("Error: {e}"),
            }
        }
        Commands::ClearDone => {
            match clear_done() {
                Ok(n) => println!("Removed {n} completed task(s)."),
                Err(e) => eprintln!("Error: {e}"),
            }
        }
    }
}