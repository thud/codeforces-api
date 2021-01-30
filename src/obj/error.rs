#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    Parse(reqwest::Error),
    CodeforcesApi(String),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Http(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Http(ref e) => write!(f, "HTTP: {}", e),
            Error::Parse(ref e) => write!(f, "Parse: {}", e),
            Error::CodeforcesApi(ref s) => write!(f, "Codeforces API: {}", s),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Http(ref e) => Some(e),
            Error::Parse(ref e) => Some(e),
            Error::CodeforcesApi(_) => None,
        }
    }
}
