
use std::io;
use std::net::SocketAddr;
use std::convert::AsRef;
use net::{ UdpSocket };
use bytes::MutBuf;
use bytes::alloc::BufferPool;
use futures::{Async, Poll};
use futures::stream::Stream;

///
/// UdpStream
/// Wraps the UdpSocket and provides a `futures::stream::Stream` implementation
///
pub struct UdpStream<S : AsRef<UdpSocket>, B : BufferPool> {
    socket: S,
    pool: B,
}

impl<S : AsRef<UdpSocket>, B : BufferPool, > UdpStream<S, B> {
    /// Creates a new UdpStream.  The Buffer pool is a factory of fixed sized
    /// buffers which is leveraged so that the UdpStream may continually produce
    /// data.
    pub fn new(socket: S, b: B) -> UdpStream<S, B> {
        UdpStream {
            socket: socket,
            pool: b,
        }
    }

    /// Return a reference to the underlying UDP socket
    pub fn socket<'a>(&'a self) -> &'a S {
        &self.socket
    }
}

impl<S : AsRef<UdpSocket>, B: BufferPool> Stream for UdpStream<S, B> {
    type Item = (B::Item, SocketAddr);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if let Async::NotReady = self.socket.as_ref().poll_read() {
            return Ok(Async::NotReady)
        }
        let mut buf = try!(self.pool.get());
        let result = unsafe { self.socket.as_ref().recv_from(buf.mut_bytes()) };
        match result {
            Ok((amt, addr)) => { 
                unsafe { buf.advance(amt)};
                Ok(Async::Ready(Some((buf, addr)))) },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                Ok(Async::NotReady)
            },
            Err(e) => Err(e)
        }
    }
}


