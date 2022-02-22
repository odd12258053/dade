pub type Result<T> = std::result::Result<T, Error>;

pub struct Error {
    err: Box<ErrorImpl>,
}

pub enum ErrorType {
    ParseError,
    ValidateError,
}

struct ErrorImpl {
    message: Box<str>,
    err_type: ErrorType,
}

impl Error {
    pub fn new(message: &str, err_type: ErrorType) -> Self {
        Error {
            err: Box::new(ErrorImpl {
                message: message.into(),
                err_type,
            }),
        }
    }
    pub fn new_parse_err(message: &str) -> Self {
        Error::new(message, ErrorType::ParseError)
    }
    pub fn new_validate_err(message: &str) -> Self {
        Error::new(message, ErrorType::ValidateError)
    }
    pub fn err_type(&self) -> ErrorType {
        self.err.err_type
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.err_type(), self.err.message)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.err_type(), self.err.message)
    }
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorType::ParseError => write!(f, "Parse Error"),
            ErrorType::ValidateError => write!(f, "Validate Error"),
        }
    }
}

impl std::fmt::Debug for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorType::ParseError => write!(f, "Parse Error"),
            ErrorType::ValidateError => write!(f, "Validate Error"),
        }
    }
}

impl Clone for ErrorType {
    fn clone(&self) -> Self {
        match self {
            ErrorType::ParseError => ErrorType::ParseError,
            ErrorType::ValidateError => ErrorType::ValidateError,
        }
    }
}

impl Copy for ErrorType {}
