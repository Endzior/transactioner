use std::env;

use csv::ReaderBuilder;

use crate::record::Record;

pub struct CSVParser {
}

impl CSVParser {
    fn read_filename_from_args(&self) -> String
    {
        let args: Vec<String> = env::args().collect();

        // let's make sure we get at least a single input file
        assert!(args.len() > 1);

        let result = args[1].clone();

        return result;
    }

    pub fn parse_records(&self)
    {
        let input_filename = self.read_filename_from_args();

        let mut reader = ReaderBuilder::new().from_path(&input_filename).unwrap();

        reader.deserialize::<Record>().for_each(|record| {
            println!("{}", &record.unwrap());
        });
    }
}