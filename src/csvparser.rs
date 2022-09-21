use std::{env, sync::mpsc::Sender};

use csv::{ReaderBuilder, Trim};

use crate::record::{Record, RecordType};

pub struct CSVParser {
    sender: Sender<Record>,
}

impl CSVParser {
    pub fn new(sender: Sender<Record>) -> Self {
        Self { sender }
    }

    fn read_filename_from_args(&self) -> String {
        let log_header = "CSVParser::read_filename_from_args";
        let args: Vec<String> = env::args().collect();

        // let's make sure we get at least a single input file
        assert!(
            args.len() > 1,
            "You need to provide a path as an argument to a csv file to get the data from"
        );

        let result = args[1].clone();
        log::debug!("{}: input_filename set as == {}", log_header, result);

        result
    }

    pub fn parse_records(&mut self) {
        let log_header = "CSVParser::parse_records";
        let input_filename = self.read_filename_from_args();

        let mut reader = ReaderBuilder::new()
            .trim(Trim::All)
            .from_path(&input_filename)
            .unwrap();

        reader.deserialize::<Record>().for_each(|record| {
            let unpacked_record = record.unwrap_or_else(|_| Record::default());
            log::debug!(
                "{}: parsed a new record == {}",
                log_header,
                &unpacked_record
            );
            self.sender.send(unpacked_record).unwrap();
        });

        log::debug!(
            "{}: all records parsed, sending Finished record",
            log_header
        );
        let finish_record = Record {
            record_type: RecordType::Finished,
            ..Record::default()
        };
        self.sender.send(finish_record).unwrap();
    }
}
