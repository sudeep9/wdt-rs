
use rmpv;
use std;
use native_tls;

error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        EncodeError(rmpv::encode::Error);
        DecodeError(rmpv::decode::Error);
        Io(std::io::Error);
        TlsError(native_tls::Error);
    }
}