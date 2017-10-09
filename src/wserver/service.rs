
use std::net::SocketAddr;
use std::io;
use tokio_service::Service;
use tokio_proto::TcpServer;
use futures::{Future, future};
use common;

pub struct Server {
    addr: SocketAddr,
}

impl Server {
    pub fn new(addr: SocketAddr) -> Self {
        Server{
            addr: addr,
        }
    }

    pub fn serve(&self) {
        TcpServer::new(common::proto::WdtProto, self.addr)
        .serve(|| {
            info!("New connection received");
            Ok(Inner)
         });
    }
}

struct Inner;

impl Service for Inner {
    type Request = common::request::Request;
    type Response = common::request::Request;
    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response, Error =  Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        info!("new service call received");
        Box::new(future::ok(req))
    }
}