
#![recursion_limit = "1024"]

#[macro_use]
extern crate log;

#[macro_use]
extern crate error_chain;

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
use futures::{Future, Stream};
use futures::sync::oneshot;
use futures::stream;
use futures::future;
use std::ops::Sub;


fn wait_for_response(count: &mut i32, s: Vec<oneshot::Receiver<codec::RevRequest>>) -> Vec<oneshot::Receiver<codec::RevRequest>> {
    let rsp_stream = stream::futures_unordered(s);
    //println!("stream count = {}", rsp_stream.len());
    let mut itr = rsp_stream.wait();
    while let Some(item) = itr.next() {
        item.and_then(|m|{
            *count += 1;
            //println!("count = {}", count);
            Ok(())
        });
    }

    let rspvec = Vec::<oneshot::Receiver<codec::RevRequest>>::new();
    rspvec
} 

fn send_val(id: u32, client: client::Client) -> errors::Result<()> {
    let data = vec![1 as u8; 4 * 1024 * 1024];
    let msg = codec::RevRequest{
        reqid: 10,
        data: data
    };

    let mut n = 0;
    let tid = utils::get_threadid();
    let mut rspvec = Vec::<oneshot::Receiver<codec::RevRequest>>::new();
    let mut count = 0;
    while n < 1000000 {
        let mut msg_clone = msg.clone();
        msg_clone.reqid += id * 1000000 + n;
        //println!("> tid={} reqid = {}, data = {}", tid, msg_clone.reqid, msg_clone.data);
        //client.call(&msg);

        let fut = client.call(msg_clone);
        rspvec.push(fut);

        if n % 100 == 0 {
            rspvec = wait_for_response(&mut count, rspvec);
        }
        
        n += 1;
    }
    //client.call(msg.clone());
    println!("Done sending!", );
    println!("wait done!, count = {}", count);

    //std::thread::sleep(std::time::Duration::from_secs(60));
    Ok(())
}

fn run_multiple_client() -> errors::Result<()> {
    let addr = "127.0.0.1:12345".parse().unwrap();
    let client = client::Client::new(addr)?;

    let start = std::time::Instant::now();

    let pool = threadpool::ThreadPool::new(5);
    for i in 0..5 {
        let client = client.clone();
        pool.execute(move ||{
            //let _ = start_chat().map_err(|e|{ 
            let _ = send_val(i, client).map_err(|e|{ 
                println!("error = {}", e);
            });
        });
    }
    pool.join();
    let end = std::time::Instant::now();
    println!("Elapsed time = {:?}", end.sub(start));

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