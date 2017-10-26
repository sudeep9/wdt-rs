
use std::fs;
use std::path::Path;
use std::io::Read;
use native_tls::{Pkcs12, TlsAcceptor, TlsConnector, Certificate};
use native_tls;
use tokio_core;
use tokio_tls;
use errors::*;

pub fn read_x509_cert(certfile: &Path) -> Result<Certificate> {
    let buf = fs::File::open(&certfile).and_then(|mut f|{
        let mut buf = Vec::new();
        let _count = f.read_to_end(&mut buf)?;
        Ok(buf)
    })?;

    let cert = Certificate::from_der(&buf)?;

    Ok(cert)
}

pub fn read_pkcs12_cert(certfile: &Path, passwd: &str) -> Result<Pkcs12> {
    let buf = fs::File::open(&certfile).and_then(|mut f|{
        let mut buf = Vec::new();
        let _count = f.read_to_end(&mut buf)?;
        Ok(buf)
    })?;

    let cert = Pkcs12::from_der(&buf, passwd).or_else(|e|{
        println!("Cert read error: {}", e);
        Err(e)
    })?;

    Ok(cert)
}


pub fn new_tls_acceptor(certfile: &Path, passwd: &str) -> Result<TlsAcceptor> {
    let cert = read_pkcs12_cert(certfile, passwd)?;

    let builder = TlsAcceptor::builder(cert)?;
    let acceptor = builder.build()?;

    Ok(acceptor)
}

pub fn new_tls_connect(certopt: Option<(&Path, &str)>, root_cert: Option<&Path>) -> Result<TlsConnector> {

    let mut builder = TlsConnector::builder()?;
    if certopt.is_some() {
        let (certfile, passwd) = certopt.unwrap();
        let cert = read_pkcs12_cert(certfile, passwd)?;
        builder.identity(cert)?;
    }

    if root_cert.is_some() {
        let cert = read_x509_cert(root_cert.unwrap())?;
        builder.add_root_certificate(cert)?;
    }

    let connector = builder.build()?;

    Ok(connector)
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