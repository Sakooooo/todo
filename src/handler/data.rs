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

const STYLE_TODO: anstyle::Style = anstyle::Style::new()
    .bold()
    .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red)));
const STYLE_INPROGRESS: anstyle::Style = anstyle::Style::new()
    .bold()
    .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow)));
const STYLE_DONE: anstyle::Style = anstyle::Style::new()
    .bold()
    .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green)));
impl std::fmt::Display for TaskState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskState::Done => write!(f, "{STYLE_DONE}DONE{STYLE_DONE:#}"),
            TaskState::InProgress => write!(f, "{STYLE_INPROGRESS}IN PROGRESS{STYLE_INPROGRESS:#}"),
            TaskState::Todo => write!(f, "{STYLE_TODO}TODO{STYLE_TODO:#}"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    pub id: u64,
    pub state: TaskState,
    pub task: String,
}
