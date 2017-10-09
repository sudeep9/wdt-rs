
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::pipeline::{ServerProto, ClientProto};
use request::*;
use std::io;
use codec;

pub struct WdtProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for WdtProto {
    type Request = Request;
    type Response = Request;

    /// `Framed<T, LineCodec>` is the return value of `io.framed(LineCodec)`
    type Transport = Framed<T, codec::WdtCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(codec::WdtCodec))
    }
}


impl<T: AsyncRead + AsyncWrite + 'static> ClientProto<T> for WdtProto {
    type Request = Request;
    type Response = Request;

    /// `Framed<T, LineCodec>` is the return value of `io.framed(LineCodec)`
    type Transport = Framed<T, codec::WdtCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(codec::WdtCodec))
    }
}