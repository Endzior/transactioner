use std::sync::mpsc::channel;

use csvparser::CSVParser;
use record::Record;

mod account;
mod calculator;
mod csvparser;
mod record;

fn main() {
    let (sender, receiver) = channel::<Record>();

    let join_thread = std::thread::spawn(move || {
        calculator::Calculator::new(receiver).run();
    });

    CSVParser::new(sender).parse_records();

    let _ = join_thread.join();
}
