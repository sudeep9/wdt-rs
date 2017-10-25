
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate bytes;
extern crate common;
extern crate futures_cpupool;
extern crate native_tls;
extern crate tokio_tls;

mod server;


fn main() {
    let addr = "0.0.0.0:12345".parse().unwrap();

    let srv = server::Server::new(addr);
    srv.serve().err().and_then(|e|{
        println!("Server shutdown with error: {}", e);
        Some(())
    });
}

