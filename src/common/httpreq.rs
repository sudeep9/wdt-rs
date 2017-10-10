use std::io::{Write, Read};
use std::io;
use std; 
use errors::*;
use bytes::BytesMut;

/*
requestHeader   = "POST /api HTTP/1.1\r\nHost: %s\r\nX-Drv-Encoding: %d\r\nContent-Length: %d\r\n\r\n"
responseHeader  = "POST /api HTTP/1.1\r\nX-Drv-Encoding: %d\r\nContent-Length: %d\r\n\r\n"
*/

//static REQ_HEADER_FMT: &'static str = "POST /api HTTP/1.1\r\nHost: {}\r\nX-Drv-Encoding: 8\r\nContent-Length: {}\r\n\r\n";
//static RSP_HEADER_FMT: &'static str = "POST /api HTTP/1.1\r\nX-Drv-Encoding: {}\r\nContent-Length: {}\r\n\r\n";

pub struct Request {
    header: String,
    data: Vec<u8>
}

impl Request {
    pub fn new(host: &str, data: Vec<u8>) -> Self {
        let header = format!(
            "POST /api HTTP/1.1\r\nHost: {}\r\nX-Drv-Encoding: 8\r\nContent-Length: {}\r\n\r\n", 
            host, 
            data.len()
        );
        Request {
            header: header,
            data: data
        }
    }

    pub fn encode<W: Write>(&self, w: &mut W) -> Result<()> {
        w.write(self.header.as_bytes())?;
        w.write(self.data.as_slice())?;
        Ok(())
    }

    /*
    pub fn decode(cur: &mut io::Cursor<&&mut BytesMut>) -> Result<()>{
        Ok(())
    }
    */

    fn read_header(src: &mut BytesMut) -> Result<(usize, Vec<usize>)> {
        let mut itr = src.iter().enumerate();
        let mut header_end = 0;
        let mut marks = Vec::<usize>::new();
        let mut indv_header_end = 0;

        while let Some((i, b)) = itr.next() {
            if *b != 0xd {
                continue;
            }

            header_end = i;
            indv_header_end = i;
            let mut ok = false;

            {
                let x = itr.next();
                ok = match x {
                    Some((j, c)) => *c == 0xa,
                    None => false,
                };
                if !ok {
                    continue;
                }
            }


            {
                let x = itr.next();
                ok = match x {
                    Some((j, c)) => *c == 0xd,
                    None => false,
                };
                if !ok {
                    marks.push(indv_header_end);
                    continue;
                }
            }

            {
                let x = itr.next();
                ok = match x {
                    Some((j, c)) => *c == 0xa,
                    None => false,
                };
                if !ok {
                    header_end = 0;
                    continue;
                }
            }

            break;
        }

        Ok((header_end, marks))
    }

    pub fn decode(src: &mut BytesMut) -> Result<()>{

        Ok(())
    }
}

//0123456
//acbcdef
//   cde

#[inline]
fn find_bytes(src: &[u8], from: usize, buf: &[u8]) -> Option<usize> {
    let mut i = from;
    let mut j = 0;

    while i < src.len() {
        if src[i] == buf[j] {
            j += 1
        }else{
            j = 0;
        }

        if j == buf.len() {
            return Some(i - buf.len() + 1);
        }
        i += 1;
    }

    None
}

#[inline]
fn get_bytes_between(src: &[u8], from: &[u8], to: &[u8]) -> Option<(usize, usize)> {
    find_bytes(src, 0, from).and_then(|start_off|{
        find_bytes(src, start_off + from.len(), to).and_then(|end_off|{
            Some((start_off + from.len(), end_off))
        })
    })
}

#[inline]
fn get_content_len(src: &[u8]) -> Option<usize> {
    let content_len_header = b"Content-Length: ";
    let newline = b"\r\n";

    get_bytes_between(src, content_len_header, newline).and_then(|(start, end)|{
        std::str::from_utf8(&src[start..end]).ok().and_then(|s|{
            s.parse::<usize>().ok()
        })
    })
}

#[cfg(test)]
mod tests {
    use errors::*;
    use bytes::BytesMut;
    use std;
    use std::io;
    use httpreq::Request;
    use httpreq;
    use httparse;
    use test::Bencher;

    #[test]
    fn decode_test() {
        let s = "POST /api HTTP/1.1\r\nHost: abc\r\nX-Drv-Encoding: 8\r\nContent-Length: 123\r\n\r\n";
        //let s = "Host: abc\r\nContent-Length: 123\r\n\r\n";
        //let mut buf = BytesMut::from(s.as_bytes());
        //Request::decode(&mut buf);
        httpreq::find_bytes(s.as_bytes(), 0, "\r\n\r\n".as_bytes()).and_then(|off|{
            println!("off = {}", off);
            Some(off)
        });
    }

    #[test]
    fn test_get_content_len() {
        let s = "POST /api HTTP/1.1\r\nHost: abc\r\nX-Drv-Encoding: 8\r\nContent-Length: 123\r\n\r\n";
        assert_eq!(httpreq::get_content_len(s.as_bytes()), Some(123));
    }

    #[bench]
    fn bench_find_bytes(b: &mut Bencher) {
        let s = "POST /api HTTP/1.1\r\nHost: abc\r\nX-Drv-Encoding: 8\r\nContent-Length: 123\r\n\r\n";
        b.iter(||{
            httpreq::find_bytes(s.as_bytes(), 0, "\r\n\r\n".as_bytes());
        });
    }

    #[bench]
    fn bench_get_content_len(b: &mut Bencher) {
        let s = "POST /api HTTP/1.1\r\nHost: abc\r\nX-Drv-Encoding: 8\r\nContent-Length: 123\r\n\r\n";

        b.iter(move ||{
            let content_len_header = b"Content-Length: ";
            let newline = b"\r\n";

            httpreq::get_content_len(s.as_bytes());
        });
        //b.bytes = s.len() as u64;
    }

}