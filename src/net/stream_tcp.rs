
use std::io;
use std::io::{Read};

use net::TcpStream as NetTcpStream;
use net::{ BufferPool, Buffer };
use futures::{Async, Poll};
use futures::stream::Stream;


///
/// TcpStream
/// Wraps the UdpSocket and provides a `futures::stream::Stream` implementation
///
pub struct TcpStream<B : BufferPool> {
    socket: NetTcpStream,
    pool: B
}

impl<B : BufferPool> TcpStream<B> {
    /// Creates a new TcpStream.  The Buffer pull is a factory of fixed sized
    /// buffers which is leveraged so that the TcpStream may continually produce
    /// data.
    pub fn new(socket: NetTcpStream, b: B) -> TcpStream<B> {
        TcpStream {
            socket: socket,
            pool: b
        }
    }
}

impl<B: BufferPool> Stream for TcpStream<B> {
    type Item = B::Item;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if let Async::NotReady = self.socket.poll_read() {
            return Ok(Async::NotReady)
        }
        let mut buf = try!(self.pool.get());
        match self.socket.read(buf.as_mut()) {
            Ok(amt) => { 
                buf.advance(amt);
                Ok(Async::Ready(Some(buf))) },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                Ok(Async::NotReady)
            },
            Err(e) => Err(e)
        }
    }
}

