
#![recursion_limit = "1024"]

#[macro_use]
extern crate log;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate futures;

extern crate common;
extern crate bytes;
extern crate tokio_io;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate threadpool;

mod client;
mod errors;

use common::utils;
use common::codec;
use futures::{Future, Sink};


fn send_val(mut client: client::Client) -> errors::Result<()> {
    let msg = codec::RevRequest{
        reqid: 10,
        data: utils::get_threadid()
    };

    let mut n = 0;
    while n < 10 {
        let mut msg_clone = msg.clone();
        msg_clone.reqid += n;
        client.call(msg_clone);
        std::thread::sleep(std::time::Duration::from_secs(1));
        n += 1;
    }
    //client.call(msg.clone());
    println!("Done sending!");
    std::thread::sleep(std::time::Duration::from_secs(60));
    Ok(())
}

fn run_multiple_client() -> errors::Result<()> {
    let addr = "127.0.0.1:12345".parse().unwrap();
    let client = client::Client::new(addr)?;

    let pool = threadpool::ThreadPool::new(5);
    for _ in 0..5 {
        let client = client.clone();
        pool.execute(move ||{
            //let _ = start_chat().map_err(|e|{ 
            let _ = send_val(client).map_err(|e|{ 
                println!("error = {}", e);
            });
        });
    }
    pool.join();

    Ok(())    
}

fn run() -> errors::Result<()> {
    run_multiple_client()
    //let _ = start_chat().or_else(|e| -> Result<(),()>{
    //    println!("err: {}", e);
    //    Ok(())
    //});
    //Ok(())
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