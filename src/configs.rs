//use anyhow::{format_err, Error};
pub use config::ConfigError;
use serde::{Deserialize, Serialize};

use deadpool_postgres::{Client, Pool, PoolError, Runtime};
use dotenv::dotenv;

#[derive(Debug, Deserialize)]
pub struct Config {
    //pub listen: String,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .unwrap()
            .try_deserialize()
    }
}
