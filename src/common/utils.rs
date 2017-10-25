
use simplelog::{Config, WriteLogger, CombinedLogger, LogLevelFilter};
use std;
use std::io::Write;
use rand;
use thread_id;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use openssl;

pub fn init_logging(filename: &str) {
    CombinedLogger::init(
        vec![
            WriteLogger::new(
                LogLevelFilter::Info,
                Config::default(),
                std::fs::File::create(filename).unwrap()
            ),
        ]
    ).unwrap();
}

pub fn random_sleep() {
    let dur = rand::random::<u8>();
    let tid = get_threadid();
    trace!("Sleeping for: tid = {}, {}ms", tid, dur);
    std::thread::sleep(std::time::Duration::from_millis(dur as u64));
}

pub fn random_buf(len: usize) -> Vec<u8> {
    let mut buf = Vec::<u8>::with_capacity(len);

    for _ in 0..len {
        let n = rand::random::<u8>();
        buf.push(n);
    }

    buf
}

pub fn get_threadid() -> String {
    format!("{}", thread_id::get())
}

pub fn sha1(buf: &[u8]) -> [u8; 20] {
    return openssl::sha::sha1(buf)
}


pub fn compress_buf(buf: &[u8], cbuf: &mut Vec<u8>) -> std::io::Result<()> {
    let mut e = ZlibEncoder::new(cbuf, Compression::Default);
    let _bytes_written = e.write(buf)?;
    let _cbuf = e.finish()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use utils::*;

    #[test]
    fn test_compress() {
        let buf = random_buf(1024 * 1024);
        let mut cbuf = Vec::<u8>::with_capacity(1024 * 1024);
        
        assert!(compress_buf(&buf, &mut cbuf).is_ok());
        assert!(cbuf.len() < buf.len());
    }
    
    #[bench]
    fn bench_compress(b: &mut Bencher) {
        let buf = random_buf((1024 * 1024) as usize);
        let mut cbuf = Vec::<u8>::with_capacity(1024 * 1024);
        
        b.iter(move ||{
            compress_buf(buf.as_ref(), &mut cbuf).and_then(|c|{
                Ok(())
            });
        });
    }
}