use crate::modeling::common::Encode;
use crate::modeling::data_types::{DataType, Value};
use std::io::{Error, Write};

#[derive(Debug)]
pub struct DLMSDate {
    year: u16,
    month: u8,
    day_of_month: u8,
    day_of_week: u8,
}

impl DLMSDate {
    pub fn new(year: u16, month: u8, day_of_month: u8, day_of_week: u8) -> Self {
        Self {
            year,
            month,
            day_of_month,
            day_of_week,
        }
    }
}

pub type Date = Value<DLMSDate>;

impl Encode for Date {
    fn encode_to<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(&self.0.year.to_be_bytes())?;
        writer.write(&self.0.month.to_be_bytes())?;
        writer.write(&self.0.day_of_month.to_be_bytes())?;
        writer.write(&self.0.day_of_week.to_be_bytes())?;
        Ok(())
    }
}

impl Into<DataType> for Date {
    fn into(self) -> DataType {
        DataType::Date(self)
    }
}
