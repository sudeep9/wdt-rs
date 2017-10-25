
use rmpv;
use std;
use native_tls;
use tokio_core;

error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        EncodeError(rmpv::encode::Error);
        DecodeError(rmpv::decode::Error);
        Io(std::io::Error);
        TlsError(native_tls::Error);
        TlsHandshakeError(native_tls::HandshakeError<tokio_core::net::TcpStream>);
    }
}