
use simplelog::{Config, WriteLogger, CombinedLogger, LogLevelFilter};
use std;
use rand;
use thread_id;

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

pub fn random_sleep() {
    let dur = rand::random::<u8>();
    let tid = get_threadid();
    trace!("Sleeping for: tid = {}, {}ms", tid, dur);
    std::thread::sleep(std::time::Duration::from_millis(dur as u64));
}

pub fn get_threadid() -> String {
    format!("{}", thread_id::get())
}