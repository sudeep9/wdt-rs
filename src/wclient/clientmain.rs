
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
extern crate threadpool;
extern crate flate2;

mod client;
mod errors;

use common::utils;
use common::codec;
use futures::{Future};
use futures::{Poll, Async};
use std::ops::Sub;
use std::io;
use std::collections::HashMap;


struct CallFuture {
    client: client::Client,
    n: u32,
    sent_count: u32,
    reqid: u32,
    rsp_count: std::rc::Rc<std::cell::RefCell<u32>>,
    //rsp_map: HashMap<usize, oneshot::Receiver<codec::RevRequest>>,
    rsp_map: HashMap<usize, Box<Future<Item=(), Error=()>>>,
    //tid: String,
}

impl CallFuture {
    fn new(c: client::Client, n: u32, reqid_start: u32) -> Self {
        CallFuture {
            client: c,
            n: n,
            sent_count: 0,
            reqid: reqid_start,
            rsp_count: std::rc::Rc::new(std::cell::RefCell::new(0)),
            rsp_map: HashMap::new(),
            //tid: utils::get_threadid(),
        }
    }

    fn poll_responses(&mut self) -> Poll<(),io::Error> {
        let mut done_id = 0 as usize;
        let mut found = false;

        for (i, fut) in &mut self.rsp_map {
            match fut.poll() {
                Ok(Async::NotReady) => {},
                Ok(Async::Ready(_)) => {
                    done_id = *i;
                    found = true;
                    break;
                },
                Err(_) => {
                    break;
                }
            }
        }

        if found {
            self.rsp_map.remove(&done_id);
        } 

        Ok(Async::NotReady)
    }
}

impl Future for CallFuture {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        //println!("polling");
        let rsp_count = *(self.rsp_count.as_ref().borrow());
        if rsp_count % 1000 == 0 {
            println!("s = {}, r = {}", self.sent_count, rsp_count);
        }
        if self.sent_count < self.n {
            if self.sent_count - rsp_count < 50 {
                let msg = codec::RevRequest{
                    reqid: self.reqid,
                    data: vec![1 as u8; 4 * 1024 * 1024], 
                };

                let rsp_count = self.rsp_count.clone();
                let fut = self.client.call(msg).then(move |res|{
                    let _ = res.and_then(|_m|{
                        //println!("< id = {}", m.reqid);
                        Ok(())
                    });
                    let mut count = rsp_count.as_ref().borrow_mut();
                    *count += 1;
                    let r: std::result::Result<(),()> = Ok(());
                    r
                });

                self.rsp_map.insert(self.reqid as usize, Box::new(fut));

                self.reqid += 1;
                self.sent_count += 1;
            }
        }else{
            if self.sent_count == rsp_count {
                return Ok(Async::Ready(()));
            }
        }

        return self.poll_responses();
    }
}


fn send_val(id: u32, client: client::Client) -> errors::Result<()> {
    let max_calls = 2000;
    let fut = CallFuture::new(client, max_calls, id * max_calls);
    let _ = fut.wait();

    println!("Done sending!");
    Ok(())
}

fn run_multiple_client() -> errors::Result<()> {
    let addr = "127.0.0.1:12345".parse().unwrap();
    //let addr = "172.16.21.109:12345".parse().unwrap();
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