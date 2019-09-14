use log::LevelFilter;
use serde::Deserialize;
use std::fmt;
use typemap::Key;

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

    #[serde(default = "default_prefix")]
    pub prefix: String,

    #[serde(default = "default_channel_name")]
    pub channel_name: String,

    pub admins: Option<Vec<String>>,

    #[serde(default = "default_log_level", with = "LevelFilterDef")]
    pub log_level: LevelFilter,
}

impl fmt::Display for Configuration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

impl Key for Configuration {
    type Value = Configuration;
}

fn default_prefix() -> String {
    "r~".to_string()
}

fn default_channel_name() -> String {
    "megachat".to_string()
}

fn default_log_level() -> LevelFilter {
    LevelFilter::Info
}
