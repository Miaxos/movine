use crate::display;
use crate::errors::{Error, Result};
use crate::migration::Migration;
use crate::plan_builder::Step;

mod config;
mod postgres;
mod sqlite;

pub use self::postgres::PostgresAdaptor;
pub use config::{Config, PostgresParams, SqliteParams};
pub use sqlite::SqliteAdaptor;

pub struct Adaptor {}

impl Adaptor {
    pub fn load() -> Result<Box<dyn DbAdaptor>> {
        let config = Config::load(&"movine.toml")?;
        match config {
            Config {
                postgres: Some(params),
                ..
            } => Ok(Box::new(PostgresAdaptor::from_params(&params)?)),
            Config {
                sqlite: Some(params),
                ..
            } => Ok(Box::new(SqliteAdaptor::from_params(&params)?)),
            _ => Err(Error::AdaptorNotFound),
        }
    }
}

pub trait DbAdaptor {
    fn init_up_sql(&self) -> &'static str;
    fn init_down_sql(&self) -> &'static str;
    fn load_migrations(&self) -> Result<Vec<Migration>>;
    fn run_up_migration(&mut self, migration: &Migration) -> Result<()>;
    fn run_down_migration(&mut self, migration: &Migration) -> Result<()>;

    fn run_migration_plan(&mut self, plan: &[(Step, &Migration)]) -> Result<()> {
        for (step, migration) in plan {
            display::print_step(&(*step, migration));
            match step {
                Step::Up => {
                    self.run_up_migration(&migration)?;
                }
                Step::Down => {
                    self.run_down_migration(&migration)?;
                }
            }
        }
        Ok(())
    }
}
