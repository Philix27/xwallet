// src/logger.rs

use log::{debug, error, info, trace, warn};
use std::fs;

use crate::config::AppEnv;

pub fn init() -> Result<(), fern::InitError> {
    let log_level = AppEnv::get_log_level()
        .parse::<log::LevelFilter>()
        .unwrap_or(log::LevelFilter::Info);

    let mut builder = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                chrono::Local::now().format("%H:%M:%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level)
        // log to stderr
        .chain(std::io::stderr());

    // also log to file if one is provided via env
    if let Ok(log_file) = AppEnv::get_log_file() {
        let log_file = fs::File::create(log_file)?;
        builder = builder.chain(log_file);
    }

    // globally apply logger
    builder.apply()?;

    trace!("TRACE output enabled");
    debug!("DEBUG output enabled");
    info!("INFO output enabled");
    warn!("WARN output enabled");
    error!("ERROR output enabled");

    Ok(())
}
