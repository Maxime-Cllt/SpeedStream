use crate::telemetry::tracing::log_level::LogLevel;
use chrono::Utc;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;
use std::sync::{Mutex, OnceLock};

/// Internal logger structure
struct InternalLogger {
    file: Mutex<std::fs::File>,
    min_level: Mutex<LogLevel>,
}

impl InternalLogger {
    fn new<P: AsRef<Path>>(file_path: P, min_level: LogLevel) -> io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;

        Ok(InternalLogger {
            file: Mutex::new(file),
            min_level: Mutex::new(min_level),
        })
    }

    fn log(&self, level: LogLevel, message: &str) -> io::Result<()> {
        let min_level = *self.min_level.lock().unwrap();
        if level < min_level {
            return Ok(());
        }

        let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f UTC");
        let log_entry = format!("[{timestamp}] {} - {message}\n", level.as_str());

        // Write to file
        let mut file = self.file.lock().unwrap();
        file.write_all(log_entry.as_bytes())?;
        file.flush()?;

        // Write to stderr for Docker/container log capture
        eprint!("{log_entry}");

        Ok(())
    }
}

/// Global logger instance
static LOGGER: OnceLock<InternalLogger> = OnceLock::new();

/// Global Logger with static methods accessible from anywhere
pub struct Logger;

impl Logger {
    /// Initialize the global logger (must be called once at application startup)
    pub fn init<P: AsRef<Path>>(file_path: P, min_level: LogLevel) -> io::Result<()> {
        let internal_logger = InternalLogger::new(file_path, min_level)?;
        LOGGER.set(internal_logger).map_err(|_| {
            io::Error::new(io::ErrorKind::AlreadyExists, "Logger already initialized")
        })?;
        Ok(())
    }

    /// Get the global logger instance (panics if not initialized)
    fn get_logger() -> &'static InternalLogger {
        LOGGER
            .get()
            .expect("Logger not initialized. Call Logger::init() first.")
    }

    /// Log a message with the specified level
    pub fn log(level: LogLevel, message: &str) -> io::Result<()> {
        Self::get_logger().log(level, message)
    }

    /// Log a trace message
    pub fn trace(message: &str) -> io::Result<()> {
        Self::log(LogLevel::Trace, message)
    }

    /// Log a debug message
    pub fn debug(message: &str) -> io::Result<()> {
        Self::log(LogLevel::Debug, message)
    }

    /// Log an info message
    pub fn info(message: &str) -> io::Result<()> {
        Self::log(LogLevel::Info, message)
    }

    /// Log a warning message
    pub fn warn(message: &str) -> io::Result<()> {
        Self::log(LogLevel::Warn, message)
    }

    /// Log an error message
    pub fn error(message: &str) -> io::Result<()> {
        Self::log(LogLevel::Error, message)
    }

    /// Log a fatal error message
    pub fn fatal(message: &str) -> io::Result<()> {
        Self::log(LogLevel::Fatal, message)
    }
}

/// Convenience macros for easier logging with format strings
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::telemetry::tracing::logger::Logger::error(&format!($($arg)*)).unwrap_or_else(|e| {
            eprintln!("Failed to write to log file: {}", e);
        });
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::telemetry::tracing::logger::Logger::info(&format!($($arg)*)).unwrap_or_else(|e| {
            eprintln!("Failed to write to log file: {}", e);
        });
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::telemetry::tracing::logger::Logger::warn(&format!($($arg)*)).unwrap_or_else(|e| {
            eprintln!("Failed to write to log file: {}", e);
        });
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::telemetry::tracing::logger::Logger::debug(&format!($($arg)*)).unwrap_or_else(|e| {
            eprintln!("Failed to write to log file: {}", e);
        });
    };
}

#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        $crate::telemetry::tracing::logger::Logger::trace(&format!($($arg)*)).unwrap_or_else(|e| {
            eprintln!("Failed to write to log file: {}", e);
        });
    };
}

#[macro_export]
macro_rules! log_fatal {
    ($($arg:tt)*) => {
        $crate::telemetry::tracing::logger::Logger::fatal(&format!($($arg)*)).unwrap_or_else(|e| {
            eprintln!("Failed to write to log file: {}", e);
        });
    };
}
