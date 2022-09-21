use std::{env, sync::mpsc::Sender};

use csv::{ReaderBuilder, Trim};

use crate::record::{Record, RecordType};

pub struct CSVParser {
    sender: Sender<Record>
}

impl CSVParser {
    pub fn new(sender: Sender<Record>) -> Self
    {
        Self { sender: sender }
    }

    fn read_filename_from_args(&self) -> String
    {
        let args: Vec<String> = env::args().collect();

        // let's make sure we get at least a single input file
        assert!(args.len() > 1, "You need to provide a path as an argument to a csv file to get the data from");

        let result = args[1].clone();

        return result;
    }

    pub fn parse_records(&mut self)
    {
        let input_filename = self.read_filename_from_args();

        let mut reader = ReaderBuilder::new().trim(Trim::All).from_path(&input_filename).unwrap();

        reader.deserialize::<Record>().for_each(|record| {
            if !record.is_err()
            {
                self.sender.send(record.unwrap()).unwrap();
            }
        });


        let mut finish_record = Record::default();
        finish_record.record_type = RecordType::Finished;

        self.sender.send(finish_record).unwrap();
    }
}
