
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate bytes;
extern crate common;

use futures::stream::Stream;
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;
use futures::{Future, Sink};
use std::io::Read;
use tokio_io::codec::Decoder;
use tokio_io::codec::{FramedRead, FramedWrite};
use common::codec;
use tokio_io::{AsyncRead, AsyncWrite};
use futures::future;

fn str_rev(s: &mut String) {
    unsafe {
        let mut v = s.as_mut_vec();
        v.reverse();
    }
}

fn main() {
    let mut core = Core::new().unwrap();
    let address = "0.0.0.0:12345".parse().unwrap();
    let listener = TcpListener::bind(&address, &core.handle()).unwrap();

    let connections = listener.incoming();
    let client_proc = connections.and_then(|(mut socket, addr)|{
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

