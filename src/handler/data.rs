use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Directory {
    pub name: String,
    pub path: String,
}

struct Task {}
