use log::{Record, Level, Metadata, LevelFilter};
use std::io::Write;
use std::fs::File;
use std::sync::RwLock;
use chrono::Local;
use colored::*;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    use crate::ConsoleLogger;

    #[test]
    fn test_1() {
        ConsoleLogger::init();
        log::info!("Hello, World!");
    }
}

pub struct ConsoleLogger {
    file: RwLock<File>,
}

lazy_static! {
    static ref LOGGER: ConsoleLogger = ConsoleLogger {
        file: RwLock::new(std::fs::OpenOptions::new().read(true).write(true).open("CONOUT$").unwrap()),
    };
}

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let _ = self.file.write().and_then(|mut file| {
                let level_string = {
                    match record.level() {
                        Level::Error => record.level().to_string().red(),
                        Level::Warn => record.level().to_string().yellow(),
                        Level::Info => record.level().to_string().cyan(),
                        Level::Debug => record.level().to_string().purple(),
                        Level::Trace => record.level().to_string().normal(),
                    }
                };

                let location = if let (Some(file_path), Some(line)) = (record.file(), record.line()) {
                    format!("{}:{}", file_path, line)
                }
                else {
                    record.module_path().unwrap_or_default().to_owned()
                };

                let fmt = format!("{} {:<20} [{:<5}]",
                                  Local::now().format("%Y-%m-%d %H:%M:%S.%3f"),
                                  location,
                                  level_string);

                let _ = writeln!(file, "{} {}", fmt, record.args());

                Ok(())
            });
        }
    }

    fn flush(&self) {}
}

impl ConsoleLogger {
    pub fn init() {
        let _ = log::set_logger(&*LOGGER).map(|_| log::set_max_level(LevelFilter::Info));
    }
}
