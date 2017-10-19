
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate bytes;
extern crate common;
extern crate futures_cpupool;

mod server;


fn main() {
    let addr = "0.0.0.0:12345".parse().unwrap();

    let srv = server::Server::new(addr);
    srv.serve();
}

