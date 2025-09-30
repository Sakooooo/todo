use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DirectoryInfo {
    pub name: String,
}

struct Task {}
