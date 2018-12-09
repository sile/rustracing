use trackable::error::ErrorKind as TrackableErrorKind;
use trackable::error::TrackableError;

/// This crate specific error type.
#[derive(Debug, Clone, TrackableError)]
pub struct Error(TrackableError<ErrorKind>);

/// The list of the possible error kinds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// Input data is invalid.
    InvalidInput,

    /// Other errors (e.g., I/O error).
    Other,
}
impl TrackableErrorKind for ErrorKind {}
