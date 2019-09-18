use cdrs::{
    cluster::session::Session, compression::Compression, error,
    load_balancing::LoadBalancingStrategy, transport::CDRSTransport,
};
use log::LevelFilter;
use serde::Deserialize;
use std::{cell::RefCell, fmt, sync::Arc};
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

/// alias type used for implementing the Deserialize trait on the Compression enum
#[derive(Deserialize)]
#[serde(remote = "Compression")]
enum CompressionDef {
    Lz4,
    Snappy,
    None,
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

    pub admins: Option<Vec<String>>,

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
    #[serde(default = "defaults::database_keyspace")]
    pub keyspace: String,

    #[serde(default = "defaults::database_compression", with = "CompressionDef")]
    pub compression: Compression,

    #[serde(default = "defaults::database_hosts")]
    pub hosts: Vec<DatabaseHost>,
}

/// a struct used to hold the configuration for each individual host
#[derive(Deserialize, fmt::Debug)]
pub struct DatabaseHost {
    pub username: Option<String>,

    pub password: Option<String>,

    #[serde(default = "defaults::database_host_host")]
    pub host: String,
}

