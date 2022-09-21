use crate::record::{Record, RecordType};



pub struct Calculator
{
}

impl Calculator
{

    pub fn calculate(&self, record: Record)
    {
        println!("{}", &record);

        if record.record_type == RecordType::Finished
        {
            return self.finish();
        }
    }

    fn finish(&self)
    {
        self.print_header();
    }

    fn print_header(&self)
    {
        println!("client, available, held, total, locked");
    }
}