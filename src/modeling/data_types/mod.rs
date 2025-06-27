use crate::modeling::data_types::date::Date;
use crate::modeling::data_types::date_time::DateTime;
use crate::modeling::data_types::time::Time;

pub mod date;
pub mod date_time;
pub mod time;

#[derive(Debug)]
pub struct Value<T>(T);

impl<T> Value<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

pub type Boolean = Value<bool>;
pub type BitString = Value<Vec<u8>>;
pub type DoubleLong = Value<i32>;
pub type DoubleLongUnsigned = Value<u32>;
pub type OctetString = Value<Vec<u8>>;
pub type VisibleString = Value<Vec<u8>>;
pub type UTF8String = Value<String>;
pub type BCD = Value<Vec<u8>>;
pub type Integer = Value<i8>;
pub type Long = Value<i16>;
pub type Unsigned = Value<u8>;
pub type LongUnsigned = Value<u16>;
pub type Long64 = Value<i64>;
pub type Long64Unsigned = Value<u64>;
pub type Enum = Value<u8>;
pub type Float32 = Value<f32>;
pub type Float64 = Value<f64>;

pub enum DataType {
    None,
    Array,
    Structure,
    Boolean(Boolean),
    BitString(BitString),
    DoubleLong(DoubleLong),
    DoubleLongUnsigned(DoubleLongUnsigned),
    OctetString(OctetString),
    VisibleString(VisibleString),
    UTF8String(UTF8String),
    BCD(BCD),
    Integer(Integer),
    Long(Long),
    Unsigned(Unsigned),
    LongUnsigned(LongUnsigned),
    CompactArray,
    Long64(Long64),
    Long64Unsigned(Long64Unsigned),
    Enum(Enum),
    Float32(Float32),
    Float64(Float64),
    DateTime(DateTime),
    Date(Date),
    Time(Time),
}

impl Into<u8> for DataType {
    fn into(self) -> u8 {
        match self {
            DataType::None => 0,
            DataType::Array => 1,
            DataType::Structure => 2,
            DataType::Boolean(_) => 3,
            DataType::BitString(_) => 4,
            DataType::DoubleLong(_) => 5,
            DataType::DoubleLongUnsigned(_) => 6,
            DataType::OctetString(_) => 9,
            DataType::VisibleString(_) => 10,
            DataType::UTF8String(_) => 12,
            DataType::BCD(_) => 13,
            DataType::Integer(_) => 15,
            DataType::Long(_) => 16,
            DataType::Unsigned(_) => 17,
            DataType::LongUnsigned(_) => 18,
            DataType::CompactArray => 19,
            DataType::Long64(_) => 20,
            DataType::Long64Unsigned(_) => 21,
            DataType::Enum(_) => 22,
            DataType::Float32(_) => 23,
            DataType::Float64(_) => 24,
            DataType::DateTime(_) => 25,
            DataType::Date(_) => 26,
            DataType::Time(_) => 27,
        }
    }
}
