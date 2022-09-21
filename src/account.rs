use std::collections::HashMap;

use crate::record::{Record, RecordType};

pub struct Account{
    id: u16,
    available: f64,
    held: f64,
    locked: bool,
    transactions: HashMap<u16, Record>,
    disputes: HashMap<u16, Record>,
}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {:.4}, {:.4}, {:.4}, {}", self.id, self.available, self.held, self.available + self.held, self.locked)
    }
}

impl Account {
    pub fn new(id: u16) -> Self
    {
        Self { id: id, available: 0., held: 0., locked: false, transactions: HashMap::<u16, Record>::new(), disputes: HashMap::<u16, Record>::new() }
    }

    pub fn process(&mut self, record: Record)
    {
        if self.locked
        {
            return
        }

        if !self.process_record(&record)
        {
            return
        }

        let collection = if record.record_type == RecordType::Dispute { &mut self.disputes } else { &mut self.transactions };
        collection.insert(record.trx_id, record);
    }

    fn process_record(&mut self, record: &Record) -> bool
    {
        match record.record_type {
            RecordType::Deposit => self.deposit(record),
            RecordType::Withdrawal => self.withdrawal(record),
            RecordType::Dispute => self.dispute(record),
            RecordType::Resolve => self.resolve(record),
            RecordType::Chargeback => self.chargeback(record),
            RecordType::Invalid => false,
            RecordType::Finished => false,
        }
    }

    fn deposit(&mut self, record: &Record) -> bool
    {
        self.available += record.amount.unwrap();
        true
    }

    fn withdrawal(&mut self, record: &Record) -> bool
    {
        let amount = record.amount.unwrap();
        if self.available - amount < f64::EPSILON
        {
            return false
        }

        self.available -= amount;
        true
    }

    fn dispute(&mut self, record: &Record) -> bool
    {
        let deposited = self.transactions.get(&record.trx_id);
        
        if deposited.is_none() || deposited.unwrap().record_type != RecordType::Deposit
        {
            return false
        }

        let amount = deposited.unwrap().amount.unwrap();
        self.available -= amount;
        self.held += amount;
        true
    }

    fn resolve(&mut self, record: &Record) -> bool
    {
        let disputed = self.disputes.get(&record.trx_id);

        if disputed.is_none()
        {
            return false;
        }

        // if a dispute was added -> it was already checked the deposited has correct RecordType
        let deposited = self.transactions.get(&disputed.unwrap().trx_id);
        let amount = deposited.unwrap().amount.unwrap();

        self.available += amount;
        self.held -= amount;
        true
    }

    fn chargeback(&mut self, record: &Record) -> bool
    {
        let disputed = self.disputes.get(&record.trx_id);

        if disputed.is_none()
        {
            return false;
        }

        let deposited = self.transactions.get(&disputed.unwrap().trx_id);

        self.held -= deposited.unwrap().amount.unwrap();
        self.locked = true;
        true
    }

}