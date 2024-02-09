#[derive(Debug)]
pub enum CustomError {
    ConstructError(String),
    DestructError(String),
    ParseError(String),
    RequestError(String),
    ResponseError(String),
    Error(String),
    TaskFailed,
    Unknown,
    Test,
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::ConstructError(e) => write!(f, "Failed to construct: {}", e),
            CustomError::DestructError(e) => write!(f, "Failed to destruct: {}", e),
            CustomError::ParseError(e) => write!(f, "Failed to parse: {}", e),
            CustomError::RequestError(e) => write!(f, "Failed to request: {}", e),
            CustomError::ResponseError(e) => write!(f, "Failed to get response: {}", e),
            CustomError::Error(e) => {log::error!("Error: {}", e); Ok(())},
            CustomError::TaskFailed=> write!(f, "Task Failed"),
            CustomError::Unknown => write!(f, "Unknown Error"),
            CustomError::Test => {
                log::error!("Test Error: Test Error!");
                Ok(())
            }
        }
    }
}

impl std::error::Error for CustomError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}