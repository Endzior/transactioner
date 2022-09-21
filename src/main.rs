use csvparser::CSVParser;

mod account;
mod calculator;
mod csvparser;
mod record;

fn main() {
    let mut parser = CSVParser::default();
    parser.parse_records();
}
