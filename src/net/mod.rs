//! TCP/UDP bindings for `tokio-core`
//!
//! This module contains the TCP/UDP networking types, similar to the standard
//! library, which can be used to implement networking protocols.

mod tcp;
mod udp;
mod udp_stream;

use std::io;
use std::vec;

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
    type Item : Buffer + Sized + 'static;
    fn get(&self) -> Result<Self::Item, io::Error>;
}

///
/// Buffer
/// Basic set of functionality required to allow the reading
/// and writing of socket data via streams.
///
pub trait Buffer : Sized + 'static {
    fn as_mut_slice(&mut self) -> &mut [u8];
    fn as_slice(&self) -> & [u8];
    fn advance(&mut self, usize);
}



/// VecBuffer
/// This is a 1-use buffer, meant for demonstration purposes
/// This would be neither efficient no correct for multiple
/// reads
struct VecBuffer {
    buf : Vec<u8>
}

impl Buffer for VecBuffer {
    fn as_mut_slice(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
    
    fn as_slice(&self) -> & [u8] {
        self.as_slice()
    }

    fn advance(&mut self, amt : usize) {
        unsafe { self.set_len(amt) }
    }
}

///
/// VecBufferPool
/// Yep
struct VecBufferPool {
    size : usize
}

impl VecBufferPool {
    fn new(sz : usize) -> VecBufferPool {
        VecBufferPool {
            size : sz
        }
    }
}

impl BufferPool for VecBufferPool {
    type Item = VecBufferPool;
    fn get(&self) -> Result<VecBuffer, io::Error> {
        Ok(VecBuffer{ buf : vec![0; self.size] })
    }
}
