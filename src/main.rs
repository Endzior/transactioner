use std::sync::mpsc::channel;

use env_logger::Env;

use csvparser::CSVParser;
use record::Record;

mod account;
mod calculator;
mod csvparser;
mod record;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    let log_header = "main";

    let (sender, receiver) = channel::<Record>();

    let join_thread = std::thread::spawn(move || {
        log::debug!("{}::thread: creating new Engine and calling run on it", log_header);
        calculator::Calculator::new(receiver).run();
    });

    log::debug!("{}: creating inplace a new CSVReader and calling read_file on it", log_header);
    CSVParser::new(sender).parse_records();

    log::debug!("{}: calling join_thread on the created thread, will wait for Engine to finish processing", log_header);
    let _ = join_thread.join();
}
