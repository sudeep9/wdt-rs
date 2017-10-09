
use simplelog::{Config, WriteLogger, CombinedLogger, LogLevelFilter};
use std;

pub fn init_logging(filename: &str) {
    CombinedLogger::init(
        vec![
            WriteLogger::new(
                LogLevelFilter::Info,
                Config::default(),
                std::fs::File::create(filename).unwrap()
            ),
        ]
    ).unwrap();
}
