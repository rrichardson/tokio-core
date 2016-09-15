//! TCP/UDP bindings for `tokio-core`
//!
//! This module contains the TCP/UDP networking types, similar to the standard
//! library, which can be used to implement networking protocols.

mod tcp;
mod udp;
mod stream_udp;
mod stream_tcp;

use std::io;

pub use self::tcp::{TcpStream, TcpStreamNew};
pub use self::tcp::{TcpListener, Incoming};
pub use self::udp::{UdpSocket};

/// Implementations of futures::streams for TCP and UDP
pub mod stream {
    pub use super::stream_udp::UdpStream as Udp;
    pub use super::stream_tcp::TcpStream as Tcp;
}

///
/// Buffer
/// Requires the minimal amount of functionality for futures::Streams over
/// sockets to provide a never-ending supply of buffers filled with the results
/// of Socket::read
///
pub trait Buffer : AsMut<[u8]> {
    ///After reading, use this method to re-set the end of the readable data
    ///to the new offset.
    fn advance(&mut self, usize);
}

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

    ///Something that implements the Buffer trait and constraints
    type Item : Buffer;

    /// Function which produces a new buffer on demand.  In a real server
    /// scenario, this might run out of memory, hence the possibility for
    /// an io::Error
    fn get(&self) -> Result<Self::Item, io::Error>;
}

///
/// Simple example of an implementation of Buffer
/// This is neither efficient nor correct for multiple
/// uses, don't use this in production unless you know
/// what you're doing. 
pub struct VecBuffer {
    buf : Vec<u8>
}

impl VecBuffer {

    /// Construct a new VecBuffer
    pub fn new(sz : usize) -> VecBuffer {
        VecBuffer { buf : vec![0; sz] }
    }
}


impl Buffer for VecBuffer {
    fn advance(&mut self, sz : usize) {
        unsafe { self.buf.set_len(sz) };
    }

}

impl AsMut<[u8]> for VecBuffer {
    fn as_mut(&mut self) -> &mut [u8] {
        self.buf.as_mut_slice()
    }
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
    type Item = VecBuffer;
    fn get(&self) -> Result<Self::Item, io::Error> {
        Ok(VecBuffer::new(self.size))
    }
}
