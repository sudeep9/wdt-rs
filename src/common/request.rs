
use std::io::{Write, Read};
use std::io;
use rmpv::{Value, Utf8String};
use rmpv;
use errors::*;

pub struct Request {
    pub buf: String
}

impl Request {
    pub fn new(s: &str) -> Self {
        Request{
            buf: s.to_owned(),
        }
    }

    pub fn encode<W: Write>(&self, out: &mut W) -> Result<()> {
        rmpv::encode::write_value(out, &Value::String(Utf8String::from(self.buf.as_str())))?;
        Ok(())
    }

    pub fn decode<R: Read>(rd: &mut R) -> Result<Request> {
        let val = rmpv::decode::read_value(rd)?;

        if !val.is_str() {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Expected a string"))?;
        }

        let req = Request{
            buf: val.as_str().unwrap().to_owned()
        };

        Ok(req)
    }
}



