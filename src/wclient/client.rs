
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
use common::{codec};
use futures;
use futures::sync::mpsc::{channel, Sender};
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

    pub fn call(&self, msg_clone: codec::RevRequest) -> oneshot::Receiver<codec::RevRequest> {
        //let msg_clone = msg.clone();
        let (rsptx, rsprx) = oneshot::channel::<codec::RevRequest>();
        let p = Payload{
            req: msg_clone,
            rsptx: rsptx
        };

        let tx = self.tx.clone();
        let _res = tx.send(p).wait();

        rsprx
        
        /*
        rsprx.map(|n|{
            println!("response id = {}, data = {}", n.reqid, n.data);
        }).wait();
        */
    }

    fn spawn_io_thread(addr: SocketAddr) -> io::Result<Sender<Payload>> {
        let (tx, rx) = channel::<Payload>(1);

        println!("Spawing io thread");

        let _th = std::thread::spawn(move || -> io::Result<()>{
            let mut core = Core::new().unwrap();
            println!("Creating core");
            let handle = core.handle();

            let reqmap = Rc::new(RefCell::new(HashMap::new()));
            //let reqmap: Rc<HashMap::<u32, oneshot::Sender<codec::RevRequest>>> = Rc::new(HashMap::new());

            let conn_fut = TcpStream::connect(&addr, &handle).and_then(|stream|{
                Ok(stream)
            });

            println!("About to connect");
            let stream = core.run(conn_fut)?;
            println!("Connected");

            let (rd, wr) = stream.split();
            let fw = FramedWrite::new(wr, codec::RevCodec); 
            let fr = FramedRead::new(rd, codec::RevCodec); 

            let map_clone = reqmap.clone();
            let work_stream = rx.then(move |val| -> io::Result<codec::RevRequest>{
                let p = val.ok().unwrap();

                let mut map = map_clone.as_ref().borrow_mut();
                map.insert(p.req.reqid, p.rsptx);

                Ok(p.req)
            });

            let req_stream = fw.send_all(work_stream).and_then(|_f| {
                println!("sent");
                Ok(())
            }).map_err(|_e|{
            });

            handle.spawn(req_stream);

            let map_clone = reqmap.clone();
            let read_stream = fr.and_then(move |msg|{
                //println!("## rsp id = {}, data = {}", msg.reqid, msg.data.len());
                let mut map = map_clone.as_ref().borrow_mut();
                match map.remove(&msg.reqid) {
                    Some(tx) => {
                        match tx.send(msg) {
                            Ok(_) => {},
                            Err(_) => {println!("rsp not sent");}
                        }
                    },
                    None => {println!("## id = {}, data len = {}", msg.reqid, msg.data.len());}
                };
                Ok(())
            });

            let in_stream = read_stream.for_each(|_n|{
                futures::future::result::<(), io::Error>(Ok(()))
            }).map_err(|e|{
                println!("err = {}", e);
            });

            handle.spawn(in_stream);

            core.run(futures::future::empty::<(), io::Error>())?;

            Ok(())
        });

        Ok(tx.clone())
    }
}
