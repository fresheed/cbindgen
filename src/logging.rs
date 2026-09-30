/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;
use std::sync::Mutex;

use log::*;

pub struct TraceLogger;
pub struct WarnLogger;
pub struct InfoLogger;
pub struct ErrorLogger;

impl TraceLogger {
    pub fn init() -> Result<(), SetLoggerError> {
        log::set_logger(&TraceLogger)?;
        log::set_max_level(LevelFilter::Trace);
        Ok(())
    }
}
impl log::Log for TraceLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            eprintln!("{}: {}", record.level(), record.args());
        }
    }

    fn flush(&self) {
        io::stderr().flush().unwrap();
    }
}

impl WarnLogger {
    pub fn init() -> Result<(), SetLoggerError> {
        log::set_logger(&WarnLogger)?;
        log::set_max_level(LevelFilter::Warn);
        Ok(())
    }
}
impl log::Log for WarnLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Warn
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            eprintln!("{}: {}", record.level(), record.args());
        }
    }

    fn flush(&self) {
        io::stderr().flush().unwrap();
    }
}

impl ErrorLogger {
    pub fn init() -> Result<(), SetLoggerError> {
        log::set_logger(&ErrorLogger)?;
        log::set_max_level(LevelFilter::Error);
        Ok(())
    }
}
impl log::Log for ErrorLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Error
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            eprintln!("{}: {}", record.level(), record.args());
        }
    }

    fn flush(&self) {
        io::stderr().flush().unwrap();
    }
}

impl InfoLogger {
    pub fn init() -> Result<(), SetLoggerError> {
        log::set_logger(&InfoLogger)?;
        log::set_max_level(LevelFilter::Info);
        Ok(())
    }
}
impl log::Log for InfoLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            eprintln!("{}: {}", record.level(), record.args());
        }
    }

    fn flush(&self) {
        io::stderr().flush().unwrap();
    }
}

/// Logs every level to a file (opened in append mode, so parallel cbindgen
/// processes can share it). Warnings and errors are still echoed to stderr.
pub struct FileLogger {
    file: Mutex<File>,
    tag: String,
}

impl FileLogger {
    pub fn init(path: &str, tag: String) -> Result<(), SetLoggerError> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap_or_else(|e| panic!("Failed to open log file {path}: {e}"));
        // The logger lives for the whole process, so leaking it is fine.
        let logger = Box::leak(Box::new(FileLogger {
            file: Mutex::new(file),
            tag,
        }));
        log::set_logger(logger)?;
        log::set_max_level(LevelFilter::Trace);
        Ok(())
    }
}
impl log::Log for FileLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        // Format the whole line up front so it goes out in a single write.
        let line = format!("[{}] {}: {}\n", self.tag, record.level(), record.args());
        if let Ok(mut file) = self.file.lock() {
            let _ = file.write_all(line.as_bytes());
        }
        if record.level() <= Level::Warn {
            eprintln!("{}: {}", record.level(), record.args());
        }
    }

    fn flush(&self) {
        if let Ok(mut file) = self.file.lock() {
            let _ = file.flush();
        }
    }
}
