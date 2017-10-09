
#![recursion_limit = "1024"]

#[macro_use]
extern crate log;

#[macro_use]
extern crate error_chain;

extern crate common;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate futures;

mod service;
mod errors;

use common::utils;
use tokio_core::reactor::Core;

fn chat(core: &mut Core, client: service::Client) -> errors::Result<()> {
    let rsp = client.call(core, "Hello")?;
    println!("rsp: {}", rsp);
    let rsp = client.call(core, "How are you?")?;
    println!("rsp: {}", rsp);
    let rsp = client.call(core, "What do you wish for?")?;
    println!("rsp: {}", rsp);

    Ok(())
}

fn start_chat() -> errors::Result<()> {
    let mut core = Core::new().unwrap();
    let addr = "127.0.0.1:12345".parse().unwrap();
    let client = service::Client::connect(&mut core, &addr)?;

    info!("Client connected at: {}", client.get_addr_ref());

    chat(&mut core, client)
}

fn run() -> errors::Result<()> {
    start_chat()
}

fn main() {
    utils::init_logging("client.log");

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