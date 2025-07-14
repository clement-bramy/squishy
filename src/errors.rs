pub type Result<T> = std::result::Result<T, SquishError>;

#[derive(Debug)]
pub enum SquishError {
    ScanError { message: String },
    SquishingError { message: String },
}

impl std::error::Error for SquishError {}
impl std::fmt::Display for SquishError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ScanError { message } => write!(f, "{message}"),
            Self::SquishingError { message } => write!(f, "{message}"),
        }
    }
}
