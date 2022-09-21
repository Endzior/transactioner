use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum RecordType
{
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
    Invalid,
    Finished
}

#[derive(Deserialize)]
pub struct Record
{
    #[serde(rename = "type")]
    record_type: RecordType,
    #[serde(rename = "client")]
    client_id: u16,
    #[serde(rename = "tx")]
    trx_id: u16,
    #[serde(rename = "amount")]
    amount: Option<f64>
}

impl Record
{
    pub fn default() -> Self
    {
        Self { record_type: RecordType::Invalid, client_id: u16::MAX, trx_id: u16::MAX, amount: None }
    }
}

impl std::fmt::Display for RecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecordType::Deposit => write!(f, "Deposit"),
            RecordType::Withdrawal => write!(f, "Withdrawal"),
            RecordType::Dispute => write!(f, "Dispute"),
            RecordType::Resolve => write!(f, "Resolve"),
            RecordType::Chargeback => write!(f, "Chargeback"),
            RecordType::Invalid => write!(f, "Invalid"),
            RecordType::Finished => write!(f, "Finished"),
        }
    }
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {:?}, {:?}, {:?}", self.record_type, self.client_id, self.trx_id, self.amount)
    }
}