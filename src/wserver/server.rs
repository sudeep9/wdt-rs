#![allow(dead_code)]

use std::net::SocketAddr;
use futures::{Sink, Future, Stream};
use futures_cpupool::{CpuPool};
use tokio_io::{AsyncRead};
use tokio_io::codec::{FramedRead, FramedWrite};
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use common::codec;
use std::io;

pub struct Server {
    addr: SocketAddr,
    work: CpuPool,
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
            work: CpuPool::new(5),
        }
    }

    pub fn get_addr<'a>(&'a self) -> &'a SocketAddr {
        return &self.addr;
    }

    pub fn serve(&self) -> io::Result<()> {
        let mut core = Core::new().unwrap();

        let listener = TcpListener::bind(&self.addr, &core.handle()).unwrap();

        let connections = listener.incoming();
        let client_proc = connections.and_then(|(socket, _)|{
            println!("new connection");
            let (rd, wr) = socket.split();
            let fw = FramedWrite::new(wr, codec::RevCodec); 
            let fr = FramedRead::new(rd, codec::RevCodec); 
            let processed = fr.and_then(|mut m|{
                self.work.spawn_fn(||{
                    m.data.reverse();
                    //println!("id = {}, data = {}", m.reqid, m.data);
                    Ok(m)
                })
            });

            Ok((processed, fw))
        });

        let server = client_proc.for_each(|(proc_stream, fw)| {
            fw.send_all(proc_stream).then(|_|{
                println!("rsp done");
                Ok(())
            })
        });

        core.run(server)
    }
}
