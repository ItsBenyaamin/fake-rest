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