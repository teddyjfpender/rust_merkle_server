use std::io::Write;
use log::LevelFilter;
use env_logger;

pub fn initialize_logger() {
    env_logger::Builder::new()
        .format(|buf, record| writeln!(buf, "[{}] - {}", record.level(), record.args()))
        .filter(None, LevelFilter::Info)
        .init();
}
