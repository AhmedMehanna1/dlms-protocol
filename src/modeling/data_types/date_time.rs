use crate::modeling::data_types::Value;
use crate::modeling::data_types::date::DLMSDate;
use crate::modeling::data_types::time::DLMSTime;

pub struct DLMSDateTime {
    date: DLMSDate,
    time: DLMSTime,
    deviation: i16,
    clock_status: u8,
}

pub type DateTime = Value<DLMSDateTime>;
