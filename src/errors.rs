use postgres::error::Error as PostgresError;
use rusqlite::Error as SqliteError;
use std::fmt;
use std::io;
use toml::de::Error as TomlError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ConfigNotFound,
    PgParamError {
        user: bool,
        password: bool,
        database: bool,
        host: bool,
        port: bool,
    },

    SqliteParamError {
        file: bool,
    },

    BadMigration,
    Unknown,
    AdaptorNotFound,
    IoError(io::Error),
    TomlError(TomlError),
    PgError(PostgresError),
    SqliteError(SqliteError),
    Envy(envy::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            ConfigNotFound => write!(f, "`movine.toml` config file not found and no environment variables were found."),
            BadMigration => write!(f, "Error parsing migrations."),
            Unknown => write!(f, "Unknown error occurred"),
            AdaptorNotFound => write!(f, "Could not find adaptor"),
            IoError(e) => write!(f, "IO Error: {}", e),
            TomlError(e) => write!(f, "Unable to read config file: {}", e),
            PgError(e) => write!(f, "Error in Postgres: {}", e),
            SqliteError(e) => write!(f, "Error in Sqlite: {}", e),
            Envy(e) => write!(f, "Error in loading environment variables: {}", e),
            SqliteParamError { .. } => write!(f, "Unable to load Sqlite params. Make sure you have `file` defined in your `movine.toml` or SQLITE_FILE defined as an environment variable"),
            PgParamError {
                user, password, database, host, port
            } => {
                write!(f, "Unable to load Postgres params. Please ensure you have the following defined:\nUser: {}\nPassword: {}\nDatabase: {}\nHost: {}\nPort: {}",
                    user, password, database, host, port)
            }
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

impl From<SqliteError> for Error {
    fn from(error: SqliteError) -> Self {
        Error::SqliteError(error)
    }
}

impl From<envy::Error> for Error {
    fn from(error: envy::Error) -> Self {
        Error::Envy(error)
    }
}
