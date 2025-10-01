use std::default;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DirectoryInfo {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CategoryInfo {
    pub version: u64,
    pub id: u64,
    pub latest_todo_id: u64,
}

impl Default for CategoryInfo {
    fn default() -> Self {
        Self {
            version: 1,
            id: 0,
            latest_todo_id: 0,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, clap::ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum TaskState {
    Done,
    InProgress,
    #[default]
    Todo,
}

impl std::fmt::Display for TaskState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskState::Done => write!(f, "Done"),
            TaskState::InProgress => write!(f, "In Progress"),
            TaskState::Todo => write!(f, "TODO"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    pub id: u64,
    pub state: TaskState,
    pub task: String,
}
