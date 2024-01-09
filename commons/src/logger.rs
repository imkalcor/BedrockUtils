use chrono::Local;
use colored::Colorize;
use env_logger::Builder;
use log::{Level, LevelFilter};
use std::env;
use std::io::Write;

/// This initiates our env_logger for the console and the terminal. Builds the format
/// for logging messages, defines colors to use for different log levels and adds the message
/// timestamps in the beginning of each message.
pub fn init_logger(level: LevelFilter) {
    env::set_var("RUST_BACKTRACE", "1");

    Builder::new()
        .format(|buf, record| {
            let level = record.level().to_string();

            let level = match record.level() {
                Level::Info => level.bright_cyan(),
                Level::Warn => level.yellow(),
                Level::Trace => level.purple(),
                Level::Debug => level.bright_white(),
                _ => level.bright_red(),
            };

            writeln!(
                buf,
                "{} [{}] {}",
                level,
                Local::now().format("%H:%M:%S").to_string().white(),
                record.args()
            )
        })
        .filter(None, level)
        .init();
}
