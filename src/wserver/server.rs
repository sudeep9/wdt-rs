#![allow(dead_code)]

use std;
use std::net::SocketAddr;
use futures::{Sink, Future, Stream};
use futures_cpupool::{CpuPool};
use tokio_io::{AsyncRead};
use tokio_io::codec::{FramedRead, FramedWrite};
use tokio_core::net::{TcpListener, TcpStream};
use tokio_core::reactor::Core;
use common::{codec, ssl};
use std::io;
use tokio_tls::{TlsStream, TlsAcceptorExt};

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

    pub fn wrap_socket(&self, s: TcpStream) -> io::Result<TlsStream<TcpStream>> {
        let certfile = &std::path::Path::new("./certs/wdt.pfx");
        let acc = ssl::new_tls_acceptor(certfile, "").map_err(|e|{
            io::Error::new(io::ErrorKind::Other, format!("{}", e))
        })?;
        let socket = acc.accept_async(s).wait().map_err(|e|{
            io::Error::new(io::ErrorKind::Other, format!("{}", e))
        })?;

        Ok(socket)
    }

    pub fn serve(&self) -> io::Result<()> {
        let mut core = Core::new().unwrap();

        let listener = TcpListener::bind(&self.addr, &core.handle()).unwrap();

        let connections = listener.incoming();
        let client_proc = connections.and_then(|(non_tls_socket, _)|{
            println!("new connection");
            let socket = self.wrap_socket(non_tls_socket)?;
            println!("socket wrapped");

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
