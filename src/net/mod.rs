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

///
/// BufferPool
/// futures::Streams produce a potentially never-ending supply of
/// messages. Therefore to turn things like sockets into streams
/// we need a never-ending supply of buffers which we can read
/// into to serve data.
/// The Trait which defines a creator of fixed-sized buffers
/// which implement the Buffer trait
///
pub trait BufferPool {
    /// Function which produces a new buffer on demand.  In a real server
    /// scenario, this might run out of memory, hence the possibility for
    /// an io::Error
    fn get(&self) -> Result<Vec<u8>, io::Error>;
}


///
/// VecBufferPool
/// Yep
pub struct VecBufferPool {
    size : usize
}

impl VecBufferPool {
    ///contstruct a new Buffer Pool
    pub fn new(sz : usize) -> VecBufferPool {
        VecBufferPool {
            size : sz
        }
    }
}

impl BufferPool for VecBufferPool {
    fn get(&self) -> Result<Vec<u8>, io::Error> {
        Ok(vec![0; self.size])
    }
}
