
use std::net::SocketAddr;
use std::io;
use std;
use std::thread::JoinHandle;
use tokio_core::reactor::{Core};
use tokio_core::net::TcpStream;
use futures::{Future, Sink, Stream};
use tokio_io;
use tokio_io::{AsyncWrite, AsyncRead};
use tokio_io::codec::{FramedParts, FramedRead, FramedWrite, Encoder, Decoder};
use common::{codec, utils};
use bytes;
use futures;
use futures::sync::mpsc::{channel, Sender, Receiver};
use futures::sync::oneshot;
use std::collections::HashMap;

struct Payload {
    req: codec::RevRequest,
    rsptx: oneshot::Sender<codec::RevRequest>
}

#[derive(Clone)]
pub struct Client {
    tx: Sender<Payload>,
}

impl Client {
    pub fn new(addr: SocketAddr) -> io::Result<Self> {
        let val = Client::spawn_io_thread(addr)?;        
        Ok(Client{
            tx: val,
        })
    }

    pub fn call(&self, msg: codec::RevRequest) {
        let (rsptx, rsprx) = oneshot::channel::<codec::RevRequest>();
        let p = Payload{
            req: msg,
            rsptx: rsptx
        };

        let tx = self.tx.clone();
        match tx.send(p).wait() {
            Err(e) => {println!("Send error = {}", e)},
            Ok(_) => {}
        }

        /*
        rsprx.map(|n|{
            println!("response id = {}, data = {}", n.reqid, n.data);
        }).wait();
        */
    }

    fn spawn_io_thread(addr: SocketAddr) -> io::Result<Sender<Payload>> {
        let (tx, rx) = channel::<Payload>(5);

        println!("Spawing io thread");

        let th = std::thread::spawn(move || -> io::Result<()>{
            let mut core = Core::new().unwrap();
            println!("Creating core");
            let handle = core.handle();

            let mut reqmap:HashMap::<u32, oneshot::Sender<codec::RevRequest>> = HashMap::new();

            let conn_fut = TcpStream::connect(&addr, &handle).and_then(|stream|{
                Ok(stream)
            });

            println!("About to connect");
            let stream = core.run(conn_fut)?;
            println!("Connected");

            let (rd, wr) = stream.split();
            let fw = FramedWrite::new(wr, codec::RevCodec); 
            let fr = FramedRead::new(rd, codec::RevCodec); 

            let work_stream = rx.then(|val| -> io::Result<codec::RevRequest>{
                let p = val.ok().unwrap();
                reqmap.insert(p.req.reqid, p.rsptx);
                Ok(p.req)
            });

            let req_stream = fw.send_all(work_stream).and_then(|f|{
                println!("sent");
                Ok(())
            });

            core.run(req_stream)?;

            Ok(())
        });

        Ok(tx.clone())
    }
}

/*
pub struct Client {
    pub stream: TcpStream,
    pub core: Core,
    pub tx: Option<futures::sync::mpsc::Sender<codec::RevRequest>>,
}

impl Client {
    pub fn connect(addr: &SocketAddr) -> io::Result<Client> {
        let mut core = Core::new().unwrap();
        let handle = core.handle();

        let fut = TcpStream::connect(&addr, &handle).and_then(|stream|{
            Ok(stream)
        });

        let stream = core.run(fut)?;
        Ok(Client{
            stream: stream,
            core: core,
            tx: None,
        }) 
    }

    pub fn sync_call(&mut self, msg: codec::RevRequest) -> io::Result<()>{
        let mut c = codec::RevCodec;

        let mut buf = bytes::BytesMut::new();
        buf.reserve(1024);
        c.encode(msg, &mut buf)?;
        
        let fut = tokio_io::io::write_all(&self.stream, buf).and_then(|(s, b)|{
            tokio_io::io::read(s, b).and_then(|(_, mut b, _)|{
                let mut cd = codec::RevCodec;
                cd.decode(&mut b).and_then(|rsp|{
                    if rsp.is_some() {
                        let m = rsp.unwrap();
                        println!("id = {}, data = {}", m.reqid, m.data);
                    }
                    Ok(())
                })
            })
        });

        self.core.run(fut)
    }

    pub fn spawn_io_thread(&mut self) {
        let (t, r) = futures::sync::mpsc::channel::<codec::RevRequest>(5);

        std::thread::spawn(move ||{
            let fut = r.and_then(|msg|{
                let reqid = msg.reqid;
                let mut c = codec::RevCodec;
                let mut buf = bytes::BytesMut::new();
                buf.reserve(1024);
                let res = c.encode(msg, &mut buf);

                Ok((reqid, buf))
            }).for_each(|(id, buf)|{
                println!("n = {} {}", id, buf.len());
                Ok(())
            });

            fut.wait()
        });

        self.tx = Some(t.clone())
    }

    pub fn call(&self, msg: codec::RevRequest) {
        let tx = self.tx.as_ref().unwrap().clone();
        let tid = utils::get_threadid();
        let res = tx.send(msg).wait();
    }
}
*/