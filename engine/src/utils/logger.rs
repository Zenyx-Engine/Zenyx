use colored::Colorize;
use log::{Level, Log, Metadata, Record};
use once_cell::sync::Lazy;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

pub static LOGGER: Lazy<DynamicLogger> = Lazy::new(DynamicLogger::new);

// A logger that dynamically switches between file and stdout
pub struct DynamicLogger {
    pub writer: Arc<Mutex<Box<dyn Write + Send>>>,
}

impl DynamicLogger {
    pub fn new() -> Self {
        Self {
            writer: Arc::new(Mutex::new(Box::new(io::stdout()))),
        }
    }

    pub fn write_to_file(&self, file_path: &str) {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_path)
            .expect("Failed to open log file");

        *self.writer.lock().unwrap() = Box::new(file);
    }

    pub fn write_to_stdout(&self) {
        *self.writer.lock().unwrap() = Box::new(io::stdout());
    }

    fn colorize_level(level: Level) -> colored::ColoredString {
        match level {
            Level::Error => "ERROR".red(),
            Level::Warn => "WARN".yellow(),
            Level::Info => "INFO".green(),
            Level::Debug => "DEBUG".blue(),
            Level::Trace => "TRACE".cyan(),
        }
    }
}

impl Log for DynamicLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut writer = self.writer.lock().unwrap();
            let level = Self::colorize_level(record.level()); // Apply coloring
            writeln!(
                writer,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                level,
                record.args()
            )
            .unwrap();
        }
    }

    fn flush(&self) {
        let mut writer = self.writer.lock().unwrap();
        writer.flush().unwrap();
    }
}
