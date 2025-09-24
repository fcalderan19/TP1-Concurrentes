use std::fmt;

/// Error handling module for the application.
/// This module defines the `Error` and `ErrorType` structs to represent different types of errors that can occur during the execution of the program.
/// 
/// The `Error` struct contains an `ErrorType` and a message string.
/// The `ErrorType` enum defines various error types such as `InputError`, `IOError`, `InvalidPath` and `ColumnsError`.
/// 
/// The `fmt::Display` trait is implemented for both `ErrorType` and `Error` to provide a user-friendly
/// string representation of the errors.
/// 
/// The Error structs has:
/// 
/// - `type_error`: ErrorType. An enum representing the type of error.
/// - `message`: String. A string containing a detailed error message.


#[derive(Debug)]
pub enum ErrorType {
    InputError,
    IOError,
    InvalidPath,
    ColumnsError,
}

#[derive(Debug)]
pub struct Error {
    pub type_error: ErrorType,
    pub message: String,
}


impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_str = match self {
            ErrorType::InputError => "Input Error",
            ErrorType::IOError => "IO Error",
            ErrorType::InvalidPath => "Invalid Path Error",
            ErrorType::ColumnsError => "Columns Error",
        };
        write!(f, "{}", type_str)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]: {}", self.type_error, self.message)
    }
}