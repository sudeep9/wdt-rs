
use std::fs;
use std::path::Path;
use std::io::Read;
use native_tls::{Pkcs12, TlsAcceptor};
use errors::*;


pub fn read_pkcs12_cert(certfile: &Path, passwd: &str) -> Result<Pkcs12> {
    let buf = fs::File::open(&certfile).and_then(|mut f|{
        let mut buf = Vec::new();
        let _count = f.read_to_end(&mut buf)?;
        Ok(buf)
    })?;

    let cert = Pkcs12::from_der(&buf, passwd)?;

    Ok(cert)
}


pub fn new_tls_acceptor(certfile: &Path, passwd: &str) -> Result<TlsAcceptor> {
    let cert = read_pkcs12_cert(certfile, passwd)?;

    let builder = TlsAcceptor::builder(cert)?;
    let acceptor = builder.build()?;

    Ok(acceptor)
}

#[cfg(test)]

mod tests {
    use ssl::*;
    use std::path::Path;

    #[test]
    fn read_cert() {
        let certfile = Path::new("./certs/wdt.pfx");
        assert!(read_pkcs12_cert(&certfile, "").is_ok());
    }

    #[test]
    fn create_acceptor() {
        let certfile = &Path::new("./certs/wdt.pfx");
        let acc = new_tls_acceptor(certfile, "").unwrap();
    }
}