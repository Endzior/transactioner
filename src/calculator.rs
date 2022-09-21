use std::collections::HashMap;

use crate::{record::{Record, RecordType}, account::Account};



pub struct Calculator
{
    accounts: HashMap<u16, Account>
}

impl Calculator
{
    pub fn default() -> Self{
        Self { accounts: HashMap::<u16, Account>::new() }
    }

    pub fn calculate(&mut self, record: Record)
    {
        if record.record_type == RecordType::Finished
        {
            return self.finish();
        }

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