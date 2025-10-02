#[derive(Debug)]
pub enum CommonErrors {
    NoFolders,
    FolderNotFound,
    CategoryNotFound,
    TaskNotFound,
}

impl std::fmt::Display for CommonErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoFolders => write!(f, "You have no folders!"),
            Self::TaskNotFound => write!(f, "Could not find task!"),
            Self::CategoryNotFound => write!(f, "Could not find category!"),
            Self::FolderNotFound => write!(f, "Could not find folder!"),
        }
    }
}

impl std::error::Error for CommonErrors {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
