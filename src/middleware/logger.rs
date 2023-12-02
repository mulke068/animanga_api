use std::{fs::OpenOptions, io::Write};

use log::Metadata;

struct FileLogger;

impl log::Log for FileLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            if record.level() <= log::Level::Debug {
                println!("[{}] - {}", record.level(), record.args());
            };
            if record.level() <= log::Level::Info {
                if let Ok(mut file) = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open("info.log")
                {
                    writeln!(&mut file, "{} - {}", record.level(), record.args()).unwrap();
                }
            };
        }
    }

    fn flush(&self) {}
}

pub fn setup_logger() {
    log::set_logger(&FileLogger).unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    // env_logger::init();
}
