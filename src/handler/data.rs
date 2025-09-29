use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Directory {
    pub name: String,
    pub path: String,
}

struct Task {}
