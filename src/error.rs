use std::fmt::Display;

pub type FakeRestResult = Result<(), Error>;
pub type RequestParseResult = Result<crate::server::request::Request, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    ConfigParsingError(String),
    ConfigFileOpenError(String),
    ConfigRequiredQueriesError(String),
    ConfigRequiredHeadersError(String),
    ParsingError(String),
    UTF8Error(String),
    IoError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConfigParsingError(e) => write!(f, "failed to parse a header! {}", e),
            Error::ConfigRequiredQueriesError(e) => write!(f, "{}", e),
            Error::ConfigRequiredHeadersError(e) => write!(f, "{}", e),
            Error::ParsingError(e) => write!(f, "{}", e),
            Error::UTF8Error(e) => write!(f, "{}", e),
            Error::IoError(e) => write!(f, "{}", e),
            Error::ConfigFileOpenError(e) => write!(f, "failed to open config file! {}", e),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e.to_string())
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Error::UTF8Error(e.to_string())
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::ConfigParsingError(e.to_string())
    }
}