
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

mod client;
mod errors;

use common::utils;
use common::codec;


fn start_chat() -> errors::Result<()> {
    let addr = "127.0.0.1:12345".parse().unwrap();
    let mut client = client::Client::connect(&addr)?;

    let msg = codec::RevRequest{reqid: 10, data: "Hello".to_owned()};
    client.call(msg)?;

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