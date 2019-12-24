use postgres::error::Error as PostgresError;
use std::fmt;
use std::io;
use toml::de::Error as TomlError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ConfigFileNotFound,
    BadMigration,
    Unknown,
    MissingParams(String),
    AdaptorNotFound(String),
    IoError(io::Error),
    TomlError(TomlError),
    PgError(PostgresError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            ConfigFileNotFound => write!(f, "Config file not found."),
            BadMigration => write!(f, "Error parsing migrations."),
            Unknown => write!(f, "Unknown error occurred"),
            MissingParams(s) => write!(f, "Missing param {}", s),
            AdaptorNotFound(db) => write!(f, "Could not find adaptor for {}", db),
            IoError(e) => write!(f, "IO Error: {}", e),
            TomlError(e) => write!(f, "Unable to read config file: {}", e),
            PgError(e) => write!(f, "Error in Postgres: {}", e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<TomlError> for Error {
    fn from(error: TomlError) -> Self {
        Error::TomlError(error)
    }
}

impl From<PostgresError> for Error {
    fn from(error: PostgresError) -> Self {
        Error::PgError(error)
    }
}
