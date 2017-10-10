
use std::net::SocketAddr;
use std::io;
use tokio_service::{Service, NewService};
use tokio_proto::TcpServer;
use futures_cpupool::{CpuPool, CpuFuture};
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
        let srv = TcpServer::new(common::proto::WdtProto, self.addr);
        srv.serve(Inner::new(5));
    }
}

struct Inner {
    pool: CpuPool
}

impl Inner {
    fn new(pool_size: usize) -> Self {
        Inner {
            pool: CpuPool::new(pool_size)
        }
    }
}

impl Service for Inner {
    type Request = common::request::Request;
    type Response = common::request::Request;
    type Error = io::Error;
    //type Future = Box<Future<Item = Self::Response, Error =  Self::Error>>;
    type Future = CpuFuture<Self::Response, Self::Error>;

    fn call(&self, mut req: Self::Request) -> Self::Future {
        info!("new service call received tid = {}", common::utils::get_threadid());
        //common::utils::random_sleep();
        //std::thread::sleep(std::time::Duration::from_millis(1));
        self.pool.spawn_fn(move || -> io::Result<common::request::Request> {
            //std::thread::sleep(std::time::Duration::from_millis(1));
            unsafe {
                for _ in 0..1000 {
                    req.buf.as_mut_vec().reverse();
                }
            }
            Ok(req)
        })
        //----------
        /*
        unsafe {
            for _ in 0..1000 {
                req.buf.as_mut_vec().reverse();
            }
        }
        Box::new(future::ok(req))
        */
    }
}

impl NewService for Inner {
    type Request = common::request::Request;
    type Response = common::request::Request;
    type Error = io::Error;
    type Instance = Inner;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(Inner::new(5))
    }
}