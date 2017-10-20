
use tokio_io::codec::{Encoder, Decoder};
use request::*;
use std::io;
use bytes::{BufMut, BytesMut, LittleEndian};
use byteorder;
use byteorder::ByteOrder;

#[derive(Clone)]
pub struct RevRequest {
    pub reqid: u32,
    pub data: String
}

pub struct RevCodec;

impl Encoder for RevCodec {
    type Item = RevRequest;
    type Error = io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        buf.put_u32::<LittleEndian>(msg.reqid);
        let data = msg.data.as_bytes();
        buf.put_u32::<LittleEndian>(data.len() as u32);
        buf.put_slice(data);
        Ok(())
    }
}

impl RevCodec {
    fn decode_inner(&mut self, src: &mut BytesMut) -> (Option<RevRequest>, u32) {
        let refbuf = src.as_ref();
        let reqid = byteorder::LittleEndian::read_u32(&refbuf);
        let data_len = byteorder::LittleEndian::read_u32(&refbuf[4..]);
        let rem_len = refbuf[8..].len();
        if (rem_len as u32) < data_len {
            return (None, 0);
        }
        
        let string_bytes = &refbuf[8..(8 + data_len as usize)];

        let data = String::from_utf8(Vec::from(string_bytes)).unwrap();

        let msg = RevRequest {
            reqid: reqid,
            data: data,
        };

        (Some(msg), 8 + data_len)
    }
}

impl Decoder for RevCodec {
    type Item = RevRequest;
    type Error = io::Error;


    fn decode(&mut self, src: &mut BytesMut) -> io::Result<Option<Self::Item>> {
        if src.len() < 8 {
            return Ok(None);
        }

        let (msgopt, bytes_read) = self.decode_inner(src);
        if msgopt.is_none() {
            return Ok(None);
        }

        src.split_to(bytes_read as usize);

        Ok(msgopt)
    }
}

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

            let mut final_pos = cur.position();
            match Request::decode(&mut cur) {
                Ok(req) => {
                    ret = Ok(Some(req));
                    final_pos = cur.position();
                },
                Err(e) => {
                    let errmsg = format!("{}", e);
                    error!("codec: decode error: {}", errmsg.as_str());
                    //ret = Err(io::Error::new(io::ErrorKind::InvalidData, errmsg));
                    ret = Ok(None);
                }
            }

            final_pos as usize    
        };

        src.split_to(pos);

        ret
    }
}