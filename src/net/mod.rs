//! TCP/UDP bindings for `tokio-core`
//!
//! This module contains the TCP/UDP networking types, similar to the standard
//! library, which can be used to implement networking protocols.

mod tcp;
mod udp;
mod udp_stream;

use std::io;

pub use self::tcp::{TcpStream, TcpStreamNew};
pub use self::tcp::{TcpListener, Incoming};
pub use self::udp::{UdpSocket};
pub use self::udp_stream::UdpStream;

pub trait BufferPool {
    fn get(&self) -> Result<Vec<u8>, io::Error>;
}

struct DefaultBufferPool {
    size : usize
}

impl DefaultBufferPool {
    fn new(sz : usize) -> DefaultBufferPool {
        DefaultBufferPool {
            size : sz
        }
    }
}

impl BufferPool for DefaultBufferPool {
    fn get(&self) -> Vec<u8> {
        Vec::with_capacity(self.size)
    }
}
