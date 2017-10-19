#![allow(dead_code)]

use std::net::SocketAddr;
use futures::{Sink, Future, Stream};
//use futures_cpupool::{CpuPool, CpuFuture};
use tokio_io::{AsyncRead};
use tokio_io::codec::{FramedRead, FramedWrite};
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use common::codec;

pub struct Server {
    addr: SocketAddr,
}

fn str_rev(s: &mut String) {
    unsafe {
        let v = s.as_mut_vec();
        v.reverse();
    }
}

impl Server {
    pub fn new(addr: SocketAddr) -> Self {
        Server{
            addr: addr,
        }
    }

    pub fn get_addr<'a>(&'a self) -> &'a SocketAddr {
        return &self.addr;
    }

    pub fn serve(&self) {
        let mut core = Core::new().unwrap();

        let listener = TcpListener::bind(&self.addr, &core.handle()).unwrap();

        let connections = listener.incoming();
        let client_proc = connections.and_then(|(socket, _)|{
            let (rd, wr) = socket.split();
            let fw = FramedWrite::new(wr, codec::RevCodec); 
            let fr = FramedRead::new(rd, codec::RevCodec); 
            let processed = fr.and_then(|mut m|{
                str_rev(&mut m.data);
                println!("id = {}", m.reqid);
                Ok(m)
            });

            Ok((processed, fw))
        });

        let server = client_proc.for_each(|(proc_stream, fw)|{
            fw.send_all(proc_stream).and_then(|_|{
                Ok(())
            })
        });
        core.run(server).unwrap();
    }
}
