use std::collections::HashMap;

use crate::record::{Record, RecordType};

pub struct Account {
    id: u16,
    available: f64,
    held: f64,
    locked: bool,
    transactions: HashMap<u16, Record>,
    disputes: HashMap<u16, Record>,
}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {:.4}, {:.4}, {:.4}, {}",
            self.id,
            self.available,
            self.held,
            self.total(),
            self.locked
        )
    }
}

impl Account {
    pub fn new(id: u16) -> Self {
        Self {
            id,
            available: 0.,
            held: 0.,
            locked: false,
            transactions: HashMap::<u16, Record>::new(),
            disputes: HashMap::<u16, Record>::new(),
        }
    }

    fn total(&self) -> f64 {
        self.available + self.held
    }

    pub fn process(&mut self, record: Record) {
        let log_header = "Account::process";
        if self.locked {
            log::debug!("{}: Account is locked, record == {}", log_header, &record);
            return;
        }

        if !self.process_record(&record) {
            log::debug!(
                "{}: record could not've been processed, record == {}",
                log_header,
                &record
            );
            return;
        }

        let collection = if record.record_type == RecordType::Dispute
            || record.record_type == RecordType::Resolve
        {
            &mut self.disputes
        } else {
            &mut self.transactions
        };
        log::debug!(
            "{}: Record has been processed - inserting into collection, record == {}",
            log_header,
            &record
        );
        collection.insert(record.trx_id, record);
    }

    fn process_record(&mut self, record: &Record) -> bool {
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

    fn deposit(&mut self, record: &Record) -> bool {
        let log_header = "Account::deposit";
        if record.amount.is_none() {
            log::debug!("{}: amount in record is None, skipping", log_header);
            return false;
        }

        let amount = record.amount.unwrap();
        log::debug!(
            "{}: old available == {}, new available == {}",
            log_header,
            self.available,
            self.available + amount
        );
        self.available += amount;
        true
    }

    fn withdrawal(&mut self, record: &Record) -> bool {
        let log_header = "Account::withdrawal";
        if record.amount.is_none() {
            log::debug!("{}: amount in record is None, skipping", log_header);
            return false;
        }

        let amount = record.amount.unwrap();
        if self.available - amount < 0. {
            log::debug!(
                "{}: available is smaller then supplied amount, available == {}, amount == {}",
                log_header,
                self.available,
                amount
            );
            return false;
        }

        log::debug!(
            "{}: old available == {}, new available == {}",
            log_header,
            self.available,
            self.available - amount
        );
        self.available -= amount;
        true
    }

    fn dispute(&mut self, record: &Record) -> bool {
        let log_header = "Account::dispute";
        let deposited = self.transactions.get(&record.trx_id);

        if deposited.is_none() || deposited.unwrap().record_type != RecordType::Deposit {
            log::debug!("{}: deposited transaction not found", log_header);
            return false;
        }

        log::debug!(
            "{}: deposited transaction found, will change available and held amounts",
            log_header
        );
        let amount = deposited.unwrap().amount.unwrap();
        log::debug!(
            "{}: old available == {}, new available == {}",
            log_header,
            self.available,
            self.available - amount
        );
        self.available -= amount;
        log::debug!(
            "{}: old held == {}, new held == {}",
            log_header,
            self.held,
            self.held + amount
        );
        self.held += amount;
        true
    }

    fn resolve(&mut self, record: &Record) -> bool {
        let log_header = "Account::resolve";
        let disputed = self.disputes.get(&record.trx_id);

        if disputed.is_none() || disputed.unwrap().record_type != RecordType::Dispute {
            log::debug!("{}: disputed transaction not found", log_header);
            return false;
        }

        log::debug!(
            "{}: disputed transaction found, will change available and held amounts",
            log_header
        );
        // if a dispute was added -> it was already checked the deposited has correct RecordType
        let deposited = self.transactions.get(&disputed.unwrap().trx_id);
        let amount = deposited.unwrap().amount.unwrap();

        log::debug!(
            "{}: old available == {}, new available == {}",
            log_header,
            self.available,
            self.available + amount
        );
        self.available += amount;
        log::debug!(
            "{}: old held == {}, new held == {}",
            log_header,
            self.held,
            self.held - amount
        );
        self.held -= amount;
        true
    }

    fn chargeback(&mut self, record: &Record) -> bool {
        let log_header = "Account::chargeback";

        let disputed = self.disputes.get(&record.trx_id);

        if disputed.is_none() || disputed.unwrap().record_type != RecordType::Dispute {
            log::debug!("{}: disputed transaction not found", log_header);
            return false;
        }

        log::debug!(
            "{}: disputed transaction found, will change available and held amounts",
            log_header
        );
        let deposited = self.transactions.get(&disputed.unwrap().trx_id);

        log::debug!(
            "{}: old held == {}, new held == {}",
            log_header,
            self.held,
            self.held - deposited.unwrap().amount.unwrap()
        );
        self.held -= deposited.unwrap().amount.unwrap();
        log::debug!("{}: locking this Account", log_header);
        self.locked = true;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_record(
        record_type: RecordType,
        client_id: u16,
        trx_id: u16,
        amount: Option<f64>,
    ) -> Record {
        Record {
            record_type: record_type,
            client_id: client_id,
            trx_id: trx_id,
            amount: amount,
        }
    }

    fn setup(record_type: RecordType) -> (f64, Account, Record) {
        let client_id = 1;
        let trx_id = 1;
        let amount = 100.;

        let account = Account::new(client_id);

        (
            amount,
            account,
            setup_record(record_type, client_id, trx_id, Some(amount)),
        )
    }

    #[test]
    fn process_deposit_increases_available_and_total() {
        let (amount, mut account, record) = setup(RecordType::Deposit);

        account.process(record);

        assert_eq!(1, account.transactions.len());
        assert!(account.disputes.is_empty());
        assert_eq!(amount, account.available);
        assert_eq!(0., account.held);
        assert_eq!(amount, account.total());
    }

    #[test]
    fn process_withdrawal_doesnt_decrease_when_no_available_funds() {
        let (_, mut account, record) = setup(RecordType::Withdrawal);

        account.process(record);

        assert!(account.transactions.is_empty());
        assert!(account.disputes.is_empty());
        assert_eq!(0., account.available);
        assert_eq!(0., account.held);
        assert_eq!(0., account.total());
    }

    #[test]
    fn process_withdrawal_decreases_available_and_total() {
        let (amount, mut account, record) = setup(RecordType::Withdrawal);
        account.available = amount;

        account.process(record);

        assert_eq!(1, account.transactions.len());
        assert!(account.disputes.is_empty());
        assert_eq!(0., account.available);
        assert_eq!(0., account.held);
        assert_eq!(0., account.total());
    }

    #[test]
    fn process_dispute_with_invalid_trx_id() {
        let (_, mut account, record) = setup(RecordType::Dispute);

        account.process(record);

        assert!(account.transactions.is_empty());
        assert!(account.disputes.is_empty());
        assert_eq!(0., account.available);
        assert_eq!(0., account.held);
    }

    #[test]
    fn process_dispute_not_a_deposit() {
        let (_, mut account, record) = setup(RecordType::Deposit);
        let trx_id = record.trx_id;
        let client_id = record.client_id;

        account.process(record);

        assert_eq!(1, account.transactions.len());
        assert!(account.disputes.is_empty());

        for record_type in [
            RecordType::Chargeback,
            RecordType::Finished,
            RecordType::Invalid,
            RecordType::Resolve,
            RecordType::Withdrawal,
            RecordType::Dispute,
        ] {
            let dispute_record = setup_record(RecordType::Dispute, client_id, trx_id, None);

            let record_in_account: &mut Record = account.transactions.get_mut(&trx_id).unwrap();
            record_in_account.record_type = record_type;

            account.process(dispute_record);
            assert_eq!(1, account.transactions.len());
            assert!(account.disputes.is_empty());
        }
    }

    #[test]
    fn process_resolve_a_dispute() {
        let (amount, mut account, deposit_record) = setup(RecordType::Deposit);
        let client_id = deposit_record.client_id;
        let trx_id = deposit_record.trx_id;

        let dispute_record = setup_record(RecordType::Dispute, client_id, trx_id, None);
        let resolve_record = setup_record(RecordType::Resolve, client_id, trx_id, None);

        account.process(deposit_record);

        assert_eq!(1, account.transactions.len());
        assert!(account.disputes.is_empty());

        assert_eq!(amount, account.available);
        assert_eq!(0., account.held);
        assert_eq!(amount, account.total());

        account.process(dispute_record);

        assert_eq!(1, account.transactions.len());
        assert_eq!(1, account.disputes.len());

        assert_eq!(0., account.available);
        assert_eq!(amount, account.held);
        assert_eq!(amount, account.total());

        account.process(resolve_record);

        assert_eq!(1, account.transactions.len());
        assert_eq!(1, account.disputes.len());

        assert_eq!(amount, account.available);
        assert_eq!(0., account.held);
        assert_eq!(amount, account.total());
    }

    #[test]
    fn process_chargeback_a_dispute_when_locked_cant_do_anything() {
        let (amount, mut account, deposit_record) = setup(RecordType::Deposit);
        let client_id = deposit_record.client_id;
        let trx_id = deposit_record.trx_id;

        let dispute_record = setup_record(RecordType::Dispute, client_id, trx_id, None);
        let chargeback_record = setup_record(RecordType::Chargeback, client_id, trx_id, None);

        account.process(deposit_record);

        assert_eq!(1, account.transactions.len());
        assert!(account.disputes.is_empty());

        assert_eq!(amount, account.available);
        assert_eq!(0., account.held);
        assert_eq!(amount, account.total());

        account.process(dispute_record);

        assert_eq!(1, account.transactions.len());
        assert_eq!(1, account.disputes.len());

        assert_eq!(0., account.available);
        assert_eq!(amount, account.held);
        assert_eq!(amount, account.total());

        account.process(chargeback_record);

        assert_eq!(1, account.transactions.len());
        assert_eq!(1, account.disputes.len());

        assert_eq!(0., account.available);
        assert_eq!(0., account.held);
        assert_eq!(0., account.total());
        assert!(account.locked);

        for record in [
            setup_record(RecordType::Deposit, client_id, trx_id + 1, Some(100.)),
            setup_record(RecordType::Withdrawal, client_id, trx_id + 1, Some(100.)),
        ] {
            account.process(record);

            assert_eq!(1, account.transactions.len());
            assert_eq!(1, account.disputes.len());

            assert_eq!(0., account.available);
            assert_eq!(0., account.held);
            assert_eq!(0., account.total());
            assert!(account.locked);
        }
    }

    #[test]
    fn process_deposit_with_none_amount() {
        let (_, mut account, mut record) = setup(RecordType::Deposit);
        record.amount = None;

        account.process(record);

        assert!(account.transactions.is_empty());
        assert!(account.disputes.is_empty());

        assert_eq!(0., account.available);
    }

    #[test]
    fn process_withdrawal_with_none_amount() {
        let (amount, mut account, mut record) = setup(RecordType::Withdrawal);
        account.available = amount;
        record.amount = None;

        account.process(record);

        assert!(account.transactions.is_empty());
        assert!(account.disputes.is_empty());

        assert_eq!(amount, account.available);
    }
}
