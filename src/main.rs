use csvparser::CSVParser;

mod csvparser;
mod record;

fn main() {
    let parser = CSVParser{};
    parser.parse_records();
}
