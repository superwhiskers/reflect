use cdrs::compression::Compression;
use log::LevelFilter;

use crate::types::{DatabaseConfig, DatabaseHost};

#[inline(always)]
pub fn prefix() -> String {
    String::from("r~")
}

#[inline(always)]
pub fn log_file() -> String {
    String::from("output.log")
}

#[inline(always)]
pub fn log_level() -> LevelFilter {
    LevelFilter::Info
}

#[inline(always)]
pub fn database_configuration() -> DatabaseConfig {
    DatabaseConfig {
        keyspace: database_keyspace(),
        compression: database_compression(),
        hosts: database_hosts(),
    }
}

#[inline(always)]
pub fn database_keyspace() -> String {
    String::from("reflect")
}

#[inline(always)]
pub fn database_compression() -> Compression {
    Compression::None
}

#[inline(always)]
pub fn database_hosts() -> Vec<DatabaseHost> {
    vec![DatabaseHost {
        username: None,
        password: None,
        host: database_host_host(),
    }]
}

#[inline(always)]
pub fn database_host_host() -> String {
    String::from("localhost:9042")
}
