
use std::net::SocketAddr;
use std::io;
use common;
use tokio_proto::TcpClient;
use tokio_proto::pipeline::ClientService;
use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;
use futures::Future;
use tokio_service::Service;
use errors;
use tokio_io;

pub struct Client {
    pub stream: TcpStream,
}

impl Client {
    pub fn connect(core: &mut Core, addr: &SocketAddr) -> io::Result<Client> {
        let handle = core.handle();
        let addr_copy = addr.clone();

        let fut = TcpStream::connect(&addr, &handle).and_then(|stream|{
            Ok(Client{
                stream:stream
            })
        });

        core.run(fut)
    }

    pub fn call(&self, core: &mut Core, s: &str) -> io::Result<usize>{
        let fut = tokio_io::io::write_all(&self.stream, s.as_bytes()).and_then(|(s, v)|{
            Ok(v.len())
        });

        core.run(fut)
    }

/*
    pub fn get_addr_ref<'a>(&'a self) -> &'a SocketAddr{
        &self.addr
    }
*/
}
