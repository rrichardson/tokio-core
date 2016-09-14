//! TCP/UDP bindings for `tokio-core`
//!
//! This module contains the TCP/UDP networking types, similar to the standard
//! library, which can be used to implement networking protocols.

mod tcp;
mod udp;
mod stream_udp;
mod stream_tcp;

use std::io;
use std::fmt::{self};
use bytes::{MutByteBuf};
use bytes::alloc::BufferPool;

pub use self::tcp::{TcpStream, TcpStreamNew};
pub use self::tcp::{TcpListener, Incoming};
pub use self::udp::{UdpSocket};


/// Implementations of futures::streams for TCP and UDP
pub mod stream {
    pub use super::stream_udp::UdpStream as Udp;
    pub use super::stream_tcp::TcpStream as Tcp;
}

///
/// ByteBufPool
/// Yep
pub struct ByteBufPool {
    size : usize
}

impl ByteBufPool {
    ///contstruct a new Buffer Pool
    pub fn new(sz : usize) -> ByteBufPool {
        ByteBufPool {
            size : sz
        }
    }
}

impl BufferPool for ByteBufPool {
    type Item = MutByteBuf;
    fn get(&self) -> Result<Self::Item, io::Error> {
        Ok(MutByteBuf::with_capacity(self.size))
    }
}


impl fmt::Debug for ByteBufPool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ByteBufPool {{ size: {} }}", self.size)
    }
}
