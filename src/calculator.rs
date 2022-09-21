use std::{collections::HashMap, sync::{mpsc::Receiver, Mutex, Arc}, os::windows::io::InvalidHandleError};

use crate::{record::{Record, RecordType}, account::Account};



pub struct Calculator
{
    receiver: Arc<Mutex<Receiver<Record>>>,
    accounts: HashMap<u16, Account>
}

impl Calculator
{
    pub fn new(receiver: Receiver<Record>) -> Self{
        Self { receiver: Arc::new(Mutex::new(receiver)), accounts: HashMap::<u16, Account>::new() }
    }

    pub fn run(&mut self)
    {
        loop
        {
            let log_header = "Calculator::run";
            log::debug!("{}: in the loop getting next record", log_header);
            let next_record = self.receiver.lock().unwrap().recv().unwrap();

            if next_record.record_type == RecordType::Invalid
            {
                continue;
            }

            if next_record.record_type == RecordType::Finished
            {
                log::debug!("{}: next_record received with record_type == RecordType::Finished", log_header);
                return self.finish();
            }
                
            self.calculate(next_record)
        }
    }

    fn calculate(&mut self, record: Record)
    {
        let log_header = "Calculator::calculate";
        log::debug!("{}: got a new record to calculate, record == {}", log_header, &record);
        let client_id = record.client_id;
        log::debug!("{}: calling process for the account from the map, record == {}", log_header, &record);
        self.accounts.entry(client_id).or_insert(Account::new(client_id)).process(record);

    }

    fn finish(&self)
    {
        self.print_header();

        for (_key, account) in &self.accounts
        {
            println!("{}", &account);
        }
    }

    fn print_header(&self)
    {
        println!("client, available, held, total, locked");
    }
}
