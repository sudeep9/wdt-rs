
use std::net::SocketAddr;
use std::io;
use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;
use futures::Future;
use tokio_io;
use tokio_io::codec::{Encoder, Decoder};
use common::codec;
use bytes;

pub struct Client {
    pub stream: TcpStream,
    pub core: Core
}

impl Client {
    pub fn connect(addr: &SocketAddr) -> io::Result<Client> {
        let mut core = Core::new().unwrap();
        let handle = core.handle();

        let fut = TcpStream::connect(&addr, &handle).and_then(|stream|{
            Ok(stream)
        });

        let stream = core.run(fut)?;
        Ok(Client{
            stream: stream,
            core: core
        }) 
    }

    pub fn call(&mut self, msg: codec::RevRequest) -> io::Result<()>{
        let mut c = codec::RevCodec;

        let mut buf = bytes::BytesMut::new();
        buf.reserve(1024);
        c.encode(msg, &mut buf)?;
        
        let fut = tokio_io::io::write_all(&self.stream, buf).and_then(|(s, b)|{
            tokio_io::io::read(s, b).and_then(|(_, mut b, _)|{
                let mut cd = codec::RevCodec;
                cd.decode(&mut b).and_then(|rsp|{
                    if rsp.is_some() {
                        let m = rsp.unwrap();
                        println!("id = {}, data = {}", m.reqid, m.data);
                    }
                    Ok(())
                })
            })
        });

        self.core.run(fut)
    }

/*
    pub fn get_addr_ref<'a>(&'a self) -> &'a SocketAddr{
        &self.addr
    }
*/
}
