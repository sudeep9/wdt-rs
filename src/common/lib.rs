#![feature(test)]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;
extern crate simplelog;
extern crate rmpv;
extern crate bytes;
extern crate tokio_io;
extern crate rand;
extern crate thread_id;
extern crate byteorder;
extern crate flate2;
extern crate test;

pub mod utils;
pub mod request;
pub mod errors;
pub mod codec;