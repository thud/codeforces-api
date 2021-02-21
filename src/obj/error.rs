/// Error type for requests sent through this crate.
///
/// When sending a request is unsuccessful, a variant of the [`Error`] type
/// will be returned.
#[derive(Debug)]
pub enum Error {
    /// `Http` errors are a wrapper for network errors returned internally by
    /// reqwest.
    ///
    /// This could be returned if, for example, the device is not connected to
    /// the internet. Further documentation can be found with the
    /// [`reqwest::Error`] type.
    Http(reqwest::Error),
    /// `Parse` errors are a wrapper for parsing errors returned internally by
    /// reqwest.
    ///
    /// This could be returned if, for example, the Codeforces API returns
    /// malformed JSON. Further documentation can be found with the
    /// [`reqwest::Error`] type.
    Parse(reqwest::Error),
    /// `CodeforcesApi` errors are returned when the Codeforces API returns a
    /// `status: FAILED` response, the comment field of the response is returned
    /// as a [`String`]
    CodeforcesApi(String),
    /// `Testcases` errors are returned only when grabbing testcases which uses
    /// webscraping internally since the Codeforces API does not provide it.
    ///
    /// For now, a simple message (`&'static str`) is returned, outlining the
    /// error. However, in future, this could/should be moved into its own enum.
    Testcases(&'static str),
}

/// Converting from a [`reqwest::Error`] is useful for quickly returning errors
/// internally.
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Http(e)
    }
}

/// Display the error with a short description of the error type as a prefix.
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Http(ref e) => write!(f, "HTTP: {}", e),
            Error::Parse(ref e) => write!(f, "Parse: {}", e),
            Error::CodeforcesApi(ref s) => write!(f, "Codeforces API: {}", s),
            Error::Testcases(ref s) => write!(f, "User: {}", s),
        }
    }
}

/// Standard error impl for custom error type.
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Http(ref e) => Some(e),
            Error::Parse(ref e) => Some(e),
            Error::CodeforcesApi(_) => None,
            Error::Testcases(_) => None,
        }
    }
}
