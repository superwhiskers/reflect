use log::LevelFilter;

#[inline]
pub fn default_prefix() -> String {
    String::from("r~")
}

#[inline]
pub fn default_log_file() -> String {
    String::from("output.log")
}

#[inline]
pub fn default_database_file() -> String {
    String::from("reflect.db")
}

#[inline]
pub fn default_log_level() -> LevelFilter {
    LevelFilter::Info
}
