
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
extern crate threadpool;

mod service;
mod errors;

use common::utils;
use tokio_core::reactor::Core;
use threadpool::ThreadPool;

fn chat(core: &mut Core, client: service::Client) -> errors::Result<()> {
    for n in 0..2500 {
        let msg = format!("{} hello {}", common::utils::get_threadid(), n);
        let rsp = client.call(core, msg.as_str())?;
        if n % 1000 == 0 {
            println!("{}", n);
        }
        //println!("rsp: {}", rsp);
    }

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
    let pool = ThreadPool::new(16);
    for _ in 0..pool.max_count() {
        pool.execute(move ||{
            let _ = start_chat().or_else(|e| -> Result<(),()>{
                println!("err: {}", e);
                Ok(())
            });
        })
    }
    pool.join();
    Ok(())
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