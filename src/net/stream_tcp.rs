
use std::io;
use std::io::{Read};

use net::TcpStream as NetTcpStream;
use bytes::{ MutBuf, BufferPool };
use futures::{Async, Poll};
use futures::stream::Stream;


///
/// TcpStream
/// Wraps the UdpSocket and provides a `futures::stream::Stream` implementation
///
pub struct TcpStream<S : AsRef<NetTcpStream>, B : BufferPool> {
    socket: S,
    pool: B,
}

impl<S : AsRef<NetTcpStream>, B : BufferPool> TcpStream<S, B> {
    /// Creates a new TcpStream.  The Buffer pool is a factory of fixed sized
    /// buffers which is leveraged so that the TcpStream may continually produce
    /// data.
    pub fn new(socket: S, b: B) -> TcpStream<S, B> {
        TcpStream {
            socket: socket,
            pool: b
        }
    }
}

impl<S : AsRef<NetTcpStream>, B: BufferPool> Stream for TcpStream<S, B> {
    type Item = B::Item;
    type Error = io::Error;

    fn poll(& mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if let Async::NotReady = self.socket.as_ref().poll_read() {
            return Ok(Async::NotReady)
        }
        let mut buf = try!(self.pool.get());
        match unsafe { self.socket.as_ref().read(buf.mut_bytes()) } {
            Ok(amt) => { 
                unsafe { buf.advance(amt) };
                Ok(Async::Ready(Some(buf))) },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                Ok(Async::NotReady)
            },
            Err(e) => Err(e)
        }
    }
}

