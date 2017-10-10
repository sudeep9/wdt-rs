

use tokio_io::codec::{Encoder, Decoder};
use request::*;
use std::io;
use bytes::{BufMut, BytesMut};

pub struct HttpCodec;