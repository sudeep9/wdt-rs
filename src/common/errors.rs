
use rmpv;
use std;
use httparse;

error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        EncodeError(rmpv::encode::Error);
        DecodeError(rmpv::decode::Error);
        Io(std::io::Error);
        ToStringError(std::string::FromUtf8Error);
        HttpParse(httparse::Error);
    }

    errors {
        NeedMoreData
            /*
        NeedMoreData(t: String) {
            description("Need more data")
            display("Need more data for {}", t)
        }
        */
    }
}