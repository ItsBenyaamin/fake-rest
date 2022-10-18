use std::fmt::Display;

pub type FakeRestResult = Result<(), Error>;
pub type RequestParseResult = Result<crate::server::request::Request, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    ConfigParsingError,
    ConfigRequiredQueriesError,
    ConfigRequiredHeadersError,
    ParsingError,
    UTF8Error,
    IoError,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConfigParsingError => write!(f, "ConfigParsingError"),
            Error::ConfigRequiredQueriesError => write!(f, "ConfigRequiredQueriesError"),
            Error::ConfigRequiredHeadersError => write!(f, "ConfigRequiredHeadersError"),
            Error::ParsingError => write!(f, "ParsingError"),
            Error::UTF8Error => write!(f, "UTF8Error"),
            Error::IoError => write!(f, "IoError"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::IoError
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(_: std::string::FromUtf8Error) -> Self {
        Error::UTF8Error
    }
}

impl From<toml::de::Error> for Error {
    fn from(_: toml::de::Error) -> Self {
        Error::ConfigParsingError
    }
}