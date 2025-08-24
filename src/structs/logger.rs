use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use chrono::{ Utc};
use crate::traits::log_level::LogLevel;

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
        let log_entry = format!("[{}] {} - {}\n", timestamp, level.as_str(), message);

        let mut file = self.file.lock().unwrap();
        file.write_all(log_entry.as_bytes())?;
        file.flush()?;
        Ok(())
    }

    fn set_min_level(&self, level: LogLevel) {
        let mut min_level = self.min_level.lock().unwrap();
        *min_level = level;
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
        LOGGER.get().expect("Logger not initialized. Call Logger::init() first.")
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

    /// Log an error with context information
    pub fn error_with_context(error: &dyn std::error::Error, context: &str) -> io::Result<()> {
        let message = format!("{}: {} (caused by: {})", context, error,
                              error.source().map_or("unknown".to_string(), |e| e.to_string()));
        Self::error(&message)
    }

    /// Log the result of an operation, logging errors if they occur
    pub fn log_result<T, E>(result: &Result<T, E>, operation: &str) -> io::Result<()>
    where
        E: std::error::Error
    {
        match result {
            Ok(_) => Self::info(&format!("Operation '{}' completed successfully", operation)),
            Err(e) => Self::error_with_context(e, &format!("Operation '{}' failed", operation)),
        }
    }

    /// Change the minimum log level
    pub fn set_min_level(level: LogLevel) {
        Self::get_logger().set_min_level(level);
    }

    /// Check if the logger is initialized
    pub fn is_initialized() -> bool {
        LOGGER.get().is_some()
    }
}

/// Convenience macros for easier logging with format strings
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::structs::logger::Logger::error(&format!($($arg)*)).unwrap_or_else(|e| {
            eprintln!("Failed to write to log file: {}", e);
        });
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::structs::logger::Logger::info(&format!($($arg)*)).unwrap_or_else(|e| {
            eprintln!("Failed to write to log file: {}", e);
        });
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::structs::logger::Logger::warn(&format!($($arg)*)).unwrap_or_else(|e| {
            eprintln!("Failed to write to log file: {}", e);
        });
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::structs::logger::Logger::debug(&format!($($arg)*)).unwrap_or_else(|e| {
            eprintln!("Failed to write to log file: {}", e);
        });
    };
}

#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        $crate::structs::logger::Logger::trace(&format!($($arg)*)).unwrap_or_else(|e| {
            eprintln!("Failed to write to log file: {}", e);
        });
    };
}

#[macro_export]
macro_rules! log_fatal {
    ($($arg:tt)*) => {
        $crate::structs::logger::Logger::fatal(&format!($($arg)*)).unwrap_or_else(|e| {
            eprintln!("Failed to write to log file: {}", e);
        });
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;

    
}