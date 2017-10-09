
use tokio_io::codec::{Encoder, Decoder};
use request::*;
use std::io;
use bytes::{BufMut, BytesMut};

pub struct WdtCodec;

impl Encoder for WdtCodec {
    type Item = Request;
    type Error = io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut BytesMut) -> io::Result<()> {
        buf.reserve(msg.buf.len() + 10);

        info!("codec: encode len = {}", msg.buf.len());
        let res = {
            match msg.encode(&mut buf.writer()) {
                Ok(()) => Ok(()),
                Err(e) => {
                    let errmsg = format!("{}", e);
                    Err(io::Error::new(io::ErrorKind::InvalidData, errmsg))
                }
            }
        };

        res
    }
}

impl Decoder for WdtCodec {
    type Item = Request;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> io::Result<Option<Self::Item>> {
        let ret: io::Result<Option<Self::Item>>;
        if src.len() == 0 {
            return Ok(None);
        } 

        info!("codec: decode len = {}", src.len());

        let pos = {
            let mut cur = io::Cursor::new(&src);

            match Request::decode(&mut cur) {
                Ok(req) => {
                    ret = Ok(Some(req));
                },
                Err(e) => {
                    let errmsg = format!("{}", e);
                    ret = Err(io::Error::new(io::ErrorKind::InvalidData, errmsg));
                }
            }

            cur.position() as usize    
        };

        src.split_to(pos);

        ret
    }
}