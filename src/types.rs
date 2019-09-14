use log::LevelFilter;
use rocksdb::DB;
use serde::Deserialize;
use std::fmt;
use typemap::Key;

use crate::defaults;

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

#[derive(Deserialize, fmt::Debug)]
pub struct Configuration {
    pub token: String,

    #[serde(default = "defaults::default_prefix")]
    pub prefix: String,

    #[serde(default = "defaults::default_log_file")]
    pub log_file: String,

    #[serde(default = "defaults::default_database_file")]
    pub database_file: String,

    pub admins: Option<Vec<String>>,

    #[serde(default = "defaults::default_log_level", with = "LevelFilterDef")]
    pub log_level: LevelFilter,
}

impl fmt::Display for Configuration {
    // TODO(superwhiskers): potentially change this to output it in a different manner. (e.g. not
    // simply being a thin proxy of the Debug trait)
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

impl Key for Configuration {
    type Value = Configuration;
}

struct Database;

impl fmt::Display for Database {
    // TODO(superwhiskers): same here
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

impl Key for Database {
    type Value = DB;
}
