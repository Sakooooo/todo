#[derive(Debug)]
pub enum CommonErrors {
    NoFolders,
}

impl std::fmt::Display for CommonErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoFolders => write!(f, "You have no folders!"),
        }
    }
}

impl std::error::Error for CommonErrors {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            _ => None,
        }
    }
}
