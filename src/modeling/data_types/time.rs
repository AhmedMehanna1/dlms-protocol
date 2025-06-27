use crate::modeling::data_types::Value;

pub struct DLMSTime {
    hour: u8,
    minute: u8,
    second: u8,
    hundredth: u8,
}

pub type Time = Value<DLMSTime>;