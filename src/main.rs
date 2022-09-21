use csvparser::CSVParser;

mod csvparser;
mod record;
mod calculator;

fn main() {
    let parser = CSVParser::default();
    parser.parse_records();
}
