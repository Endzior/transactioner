use std::{collections::HashMap, sync::{mpsc::Receiver, Mutex, Arc}};

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
            let next_record = self.receiver.lock().unwrap().recv().unwrap();

            if next_record.record_type == RecordType::Finished
            {
                return self.finish();
            }
                
            self.calculate(next_record)
        }
    }

    fn calculate(&mut self, record: Record)
    {
        let client_id = record.client_id;
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
