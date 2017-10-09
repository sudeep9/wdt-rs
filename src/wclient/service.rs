
use std::net::SocketAddr;
use std::io;
use common;
use tokio_proto::TcpClient;
use tokio_proto::pipeline::ClientService;
use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;
use futures::Future;
use tokio_service::Service;

pub struct Client {
    addr: SocketAddr,
    svc: ClientService<TcpStream, common::proto::WdtProto>
}

impl Client {
    pub fn connect(core: &mut Core, addr: &SocketAddr) -> io::Result<Client> {
        let handle = core.handle();
        let addr_copy = addr.clone();

        let fut = TcpClient::new(common::proto::WdtProto)
        .connect(addr, &handle)
        .and_then(|cs|{
            Ok(Client {addr: addr_copy,svc: cs})
        });

        core.run(fut)
    }

    pub fn get_addr_ref<'a>(&'a self) -> &'a SocketAddr{
        &self.addr
    }

    pub fn call(&self, core: &mut Core, s: &str) -> io::Result<String> {
        let req = common::request::Request::new(s);
        let fut = self.svc.call(req)
        .and_then(|r|{
            info!("reponse received: {}", r.buf);
            Ok(r.buf)
        });

        core.run(fut)
    }
}
