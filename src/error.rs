pub type Result<T> = std::result::Result<T, Error>;

pub struct Error {
    err: Box<ErrorImpl>,
}

struct ErrorImpl {
    message: Box<str>,
}

impl Error {
    pub fn new(message: &str) -> Self {
        Error {
            err: Box::new(ErrorImpl {
                message: message.into(),
            }),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err.message)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err.message)
    }
}
