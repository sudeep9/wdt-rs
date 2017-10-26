
use std::net::SocketAddr;
use std::io;
use std::rc::Rc;
use std::cell::RefCell;
use std;
use tokio_core::reactor::{Core};
use tokio_core::net::TcpStream;
use futures::{Future, Sink, Stream};
use tokio_io::{AsyncRead};
use tokio_io::codec::{FramedRead, FramedWrite};
use common::{codec, ssl};
use futures;
use futures::sync::mpsc::{channel, Sender};
use futures::sync::oneshot;
use std::collections::HashMap;
use tokio_tls::TlsConnectorExt;
use errors;

pub struct Client {
    certfile: std::path::PathBuf,
}

impl Client {
    pub fn new(certfile: &str) -> Self {
        Client {
            certfile: std::path::Path::new(certfile).to_owned(),
        }
    }

    pub fn connect(&self, addr: &SocketAddr) -> errors::Result<()> {
        let mut core = Core::new().unwrap();
        let handle = core.handle();

        let plain_conn = TcpStream::connect(&addr, &handle).and_then(move |plain_socket|{
            Ok(plain_socket)
        });

        let host = format!("{}", addr.ip());
        println!("Connecting to host: {}", host);
        let tls_handshake = plain_conn.and_then(|plain_socket|{
            //let connector = ssl::new_tls_connect(Some((&self.certfile, "1234")), Some(&std::path::Path::new("./certs/public.der"))).unwrap();
            let connector = ssl::new_tls_connect(None, None).unwrap();
            //let connector = ssl::new_tls_connect(&self.certfile, "1234").unwrap();
            connector.danger_connect_async_without_providing_domain_for_certificate_verification_and_server_name_indication(plain_socket)
            //connector.connect_async("127.0.0.1", plain_socket)
            .map_err(|e|{
                io::Error::new(io::ErrorKind::Other, format!("{}", e))
            }).and_then(|tls_sock|{
                println!("SSL handshake done");
                Ok(tls_sock)
            })
        });

        let client_proc = tls_handshake.and_then(|socket|{
            Ok(())
        });

        core.run(client_proc).or_else(|e|{
            println!("Error in client: {}", e);
            Err(e)
        })?;

        Ok(())
    }
}