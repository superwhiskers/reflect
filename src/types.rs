use log::LevelFilter;
use r2d2_redis::{r2d2::Pool, RedisConnectionManager};
use serde::Deserialize;
use std::{fmt, sync::Arc};
use typemap::Key;

use crate::defaults;

/// alias type used for implementing the Deserialize trait on the LevelFilter enum
#[derive(Deserialize)]
#[serde(remote = "LevelFilter")]
enum LevelFilterDef {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// a struct used to hold the data parsed from the configuration file
#[derive(Deserialize, fmt::Debug)]
pub struct Configuration {
    pub token: String,

    #[serde(default = "defaults::prefix")]
    pub prefix: String,

    #[serde(default = "defaults::log_file")]
    pub log_file: String,

    #[serde(default = "defaults::log_level", with = "LevelFilterDef")]
    pub log_level: LevelFilter,

    pub admins: Option<Vec<u64>>,

    #[serde(default = "defaults::database_configuration")]
    pub database: DatabaseConfig,
}

impl fmt::Display for Configuration {
    // TODO(superwhiskers): potentially change this to output it in a different manner. (e.g. not
    // simply being a thin proxy of the Debug trait)
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

impl Key for Configuration {
    type Value = Arc<Configuration>;
}

/// a struct used to hold the configuration information for the database connection
#[derive(Deserialize, fmt::Debug)]
pub struct DatabaseConfig {
    #[serde(default = "defaults::database_host")]
    pub host: String,

    #[serde(default = "defaults::database_max_connections")]
    pub max_connections: u32,
}

/// a struct used to represent the database connection object in the TypeMap
#[derive(fmt::Debug)]
pub struct Database;

impl fmt::Display for Database {
    // TODO(superwhiskers): same here
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

impl Key for Database {
    type Value = Arc<Pool<RedisConnectionManager>>;
}
