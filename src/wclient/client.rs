
use std::net::SocketAddr;
use std::io;
use std;
use std::thread::JoinHandle;
use tokio_core::reactor::{Core};
use tokio_core::net::TcpStream;
use futures::{Poll, Async, Future, Sink, Stream};
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


fn encode(msg: codec::RevRequest) -> bytes::BytesMut {
    let mut cd = codec::RevCodec;
    let mut buf = bytes::BytesMut::new();
    buf.reserve(1024);
    let res = cd.encode(msg, &mut buf);
    buf
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
        let mut fut = tx.send(p);
        fut.poll();

        /*
        match tx.send(p).wait() {
            Err(e) => {println!("Send error = {}", e)},
            Ok(_) => {}
        }
        */
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
                //reqmap.insert(p.req.reqid, p.rsptx);
                Ok(p.req)
            });

            let req_stream = fw.send_all(work_stream).and_then(|f| {
                println!("sent");
                Ok(())
            }).map_err(|e|{
            });

            handle.spawn(req_stream);

            let read_stream = fr.and_then(|msg|{
                println!("rsp id = {}, data = {}", msg.reqid, msg.data);
                Ok(())
            });

            let in_stream = read_stream.for_each(|n|{
                futures::future::result::<(), io::Error>(Ok(()))
            }).map_err(|e|{});

            handle.spawn(in_stream);

            core.run(futures::future::empty::<(), io::Error>())?;

            Ok(())
        });

        Ok(tx.clone())
    }
}
