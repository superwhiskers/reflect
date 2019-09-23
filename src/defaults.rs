use log::LevelFilter;

use crate::types::DatabaseConfig;

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
        host: database_host(),
        max_connections: database_max_connections(),
    }
}

#[inline(always)]
pub fn database_max_connections() -> u32 {
    20
}

#[inline(always)]
pub fn database_host() -> String {
    String::from("redis://localhost")
}
