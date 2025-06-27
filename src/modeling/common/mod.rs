use crate::modeling::data_types::Value;
use std::io::{Error, Read, Write};

pub trait Encode {
    fn encode_to<W: Write>(&self, writer: &mut W) -> Result<(), Error>;
}

pub trait Decode {
    fn decode_from<R: Read, T>(&self, reader: &R) -> Result<Value<T>, Error>;
}
