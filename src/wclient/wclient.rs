
#![recursion_limit = "1024"]

#[macro_use]
extern crate log;

#[macro_use]
extern crate error_chain;

extern crate common;
extern crate bytes;
extern crate tokio_io;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate futures;
extern crate threadpool;

mod service;
mod errors;

use common::utils;
use tokio_core::reactor::Core;
use threadpool::ThreadPool;
use futures::{Future, Sink};
use tokio_io::codec::{Encoder, Decoder, FramedParts, FramedRead, FramedWrite};

use common::codec;


fn start_chat() -> errors::Result<()> {
    let mut core = Core::new().unwrap();
    let addr = "127.0.0.1:12345".parse().unwrap();
    let client = service::Client::connect(&mut core, &addr)?;

    let msg = codec::RevRequest{reqid: 10, data: "Hello".to_owned()};
    /*
    let fparts = FramedParts {
        inner: client.stream,
        readbuf: bytes::BytesMut::new(),
        writebuf: bytes::BytesMut::new(),
    };
    */

    let mut c = codec::RevCodec;

    let mut buf = bytes::BytesMut::new();
    buf.reserve(1024);
    c.encode(msg, &mut buf)?;
    
    let fut = tokio_io::io::write_all(client.stream, buf).and_then(|(s, b)|{
        tokio_io::io::read(s, b).and_then(|(r, mut b, sz)|{
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


    core.run(fut)?;

    Ok(())
}

fn run() -> errors::Result<()> {
    let _ = start_chat().or_else(|e| -> Result<(),()>{
        println!("err: {}", e);
        Ok(())
    });
    Ok(())
}

fn main() {
    utils::init_logging("client.log");

    info!("Starting client");
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}