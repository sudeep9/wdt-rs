
#[macro_use]
extern crate log;
extern crate common;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate futures;

mod service;

use common::utils;

fn start() {
    let addr = "127.0.0.1:12345".parse().unwrap();
    info!("creating new server");
    let srv = service::Server::new(addr);
    info!("starting new server");
    srv.serve();
}

fn main() {
    utils::init_logging("server.log");
    start();
}
