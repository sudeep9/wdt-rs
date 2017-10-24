
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
use futures::future::{loop_fn, Loop};
use futures::{Poll, Async};
use std::ops::Sub;
use std::io;
use std::collections::HashMap;


struct CallFuture {
    client: client::Client,
    n: u32,
    sent_count: u32,
    reqid: u32,
    rsp_count: u32,
    rsp_map: HashMap<usize, oneshot::Receiver<codec::RevRequest>>,
}

impl CallFuture {
    fn new(c: client::Client, n: u32, reqid_start: u32) -> Self {
        CallFuture {
            client: c,
            n: n,
            sent_count: 0,
            reqid: reqid_start,
            rsp_count: 0,
            rsp_map: HashMap::new(),
        }
    }

    fn poll_responses(&mut self) -> Poll<(), io::Error> {
        //println!("polling responses");
        if self.rsp_count == self.sent_count && self.sent_count == self.n{
            println!("rsp_count == sent_count {}", self.rsp_count);
            return Ok(Async::Ready(()));
        }

        let mut done_fut =  Vec::<usize>::new();
        println!("rsp_map len = {}", self.rsp_map.len());
        for (i, fut) in self.rsp_map.iter_mut() {
            match fut.poll() {
                Ok(Async::NotReady) => {
                },
                Ok(Async::Ready(m)) => {
                    //println!("< id = {}, data len = {}", m.reqid, m.data.len());
                    self.rsp_count += 1;
                    done_fut.push(*i);
                    drop(m);
                },
                Err(e) => {
                    println!("Error = {}", e);
                    return Ok(Async::NotReady);
                }
            }
        }

        for i in done_fut {
            self.rsp_map.remove(&i);
        }

        return Ok(Async::NotReady);
    }
}

impl Future for CallFuture {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        //println!("polling started");
        if self.sent_count < self.n {
            if self.sent_count - self.rsp_count < 30 {
                let msg = codec::RevRequest{
                    reqid: self.reqid,
                    data: vec![1 as u8; 1024 * 1024], 
                };

                //println!("calling");
                let mut rsp_fut = self.client.call(msg);
                
                //println!("pushing in rsp map");
                self.rsp_map.insert(self.reqid as usize, rsp_fut);
                self.sent_count += 1;
                self.reqid += 1;
            }
        }
        return self.poll_responses();


        return Ok(Async::NotReady);
    }
}

fn send_val(id: u32, client: client::Client) -> errors::Result<()> {
    let data = vec![1 as u8; 1024 * 1024];
    let msg = codec::RevRequest{
        reqid: 10,
        data: data
    };

    let mut n = 0;
    let tid = utils::get_threadid();

    let mut msg_clone = msg.clone();

    let fut = CallFuture::new(client, 100000, id * 100000);

    fut.wait();
    //client.call(msg.clone());
    println!("Done sending!", );

    //std::thread::sleep(std::time::Duration::from_secs(60));
    Ok(())
}

fn run_multiple_client() -> errors::Result<()> {
    let addr = "127.0.0.1:12345".parse().unwrap();
    let client = client::Client::new(addr)?;

    let start = std::time::Instant::now();

    let pool = threadpool::ThreadPool::new(5);
    for i in 0..1 {
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