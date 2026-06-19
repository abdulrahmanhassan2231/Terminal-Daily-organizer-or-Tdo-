use serde::{Deserialize, Serialize};
use chrono::Local;
use colored::*;
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub priority: String,
    pub done: bool,
    pub created_at: String,
}

fn data_path() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("tdo");
    fs::create_dir_all(&path).ok();
    path.push("tasks.json");
    path
}

fn load_tasks() -> Vec<Task> {
    let path = data_path();
    if !path.exists() { return vec![]; }
    let data = fs::read_to_string(&path).unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_default()
}

fn save_tasks(tasks: &[Task]) -> Result<(), String> {
    let path = data_path();
    let json = serde_json::to_string_pretty(tasks).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

pub fn add_task(description: &str, priority: &str) -> Result<u32, String> {
    let mut tasks = load_tasks();
    let id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    tasks.push(Task {
        id,
        description: description.to_string(),
        priority: priority.to_string(),
        done: false,
        created_at: Local::now().format("%Y-%m-%d %H:%M").to_string(),
    });
    save_tasks(&tasks)?;
    Ok(id)
}

pub fn list_tasks(pending_only: bool) -> Result<(), String> {
    let tasks = load_tasks();
    let filtered: Vec<_> = tasks.iter()
        .filter(|t| !pending_only || !t.done)
        .collect();

    if filtered.is_empty() {
        println!("{}", "No tasks found. Add one with: tdo add <description>".dimmed());
        return Ok(());
    }

    println!("{}", "─────────────────────────────".dimmed());
    for task in &filtered {
        let status = if task.done { "done".green() } else { "todo".yellow() };
        let desc = if task.done {
            task.description.dimmed()
        } else {
            task.description.white()
        };
        println!(" [{}] #{} ({}) {}", status, task.id, task.priority, desc);
        println!("       {}", task.created_at.dimmed());
    }
    println!("{}", "─────────────────────────────".dimmed());
    println!(" {} total, {} pending",
        tasks.len(),
        tasks.iter().filter(|t| !t.done).count()
    );
    Ok(())
}

pub fn complete_task(id: u32) -> Result<(), String> {
    let mut tasks = load_tasks();
    let task = tasks.iter_mut().find(|t| t.id == id)
        .ok_or(format!("Task #{id} not found"))?;
    task.done = true;
    save_tasks(&tasks)
}

pub fn delete_task(id: u32) -> Result<(), String> {
    let mut tasks = load_tasks();
    let before = tasks.len();
    tasks.retain(|t| t.id != id);
    if tasks.len() == before {
        return Err(format!("Task #{id} not found"));
    }
    save_tasks(&tasks)
}

pub fn export_tasks(output: &str) -> Result<(), String> {
    let tasks = load_tasks();
    let date = Local::now().format("%Y-%m-%d").to_string();
    let mut md = format!("# Tasks — {date}\n\n");
    for task in &tasks {
        let check = if task.done { "x" } else { " " };
        md.push_str(&format!("- [{}] #{} {} [{}] {}\n",
            check, task.id, task.description, task.priority, task.created_at));
    }
    fs::write(output, md).map_err(|e| e.to_string())
}

pub fn clear_done() -> Result<usize, String> {
    let mut tasks = load_tasks();
    let before = tasks.len();
    tasks.retain(|t| !t.done);
    let removed = before - tasks.len();
    save_tasks(&tasks)?;
    Ok(removed)
}