use log::LevelFilter;

#[inline(always)]
pub fn prefix() -> String {
    String::from("r~")
}

#[inline(always)]
pub fn log_file() -> String {
    String::from("output.log")
}

#[inline(always)]
pub fn database_file() -> String {
    String::from("reflect.db")
}

#[inline(always)]
pub fn log_level() -> LevelFilter {
    LevelFilter::Info
}
