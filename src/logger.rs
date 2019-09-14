use chrono;
use fern;
use log::LevelFilter;

pub fn start_logging(log_level: LevelFilter) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%d-%m-%Y][%H:%M:%S]"),
                record.target(),
                record.level(),
                message,
            ))
        })
        .level(log_level)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
