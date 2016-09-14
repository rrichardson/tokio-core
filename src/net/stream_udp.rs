
use std::io;
use std::net::SocketAddr;
use net::{ UdpSocket };
use bytes::MutBuf;
use bytes::alloc::BufferPool;
use futures::{Async, Poll};
use futures::stream::Stream;

///
/// UdpStream
/// Wraps the UdpSocket and provides a `futures::stream::Stream` implementation
///
pub struct UdpStream<B : BufferPool> {
    socket: UdpSocket,
    pool: B,
}

impl<B : BufferPool> UdpStream<B> {
    /// Creates a new UdpStream.  The Buffer pool is a factory of fixed sized
    /// buffers which is leveraged so that the UdpStream may continually produce
    /// data.
    pub fn new(socket: UdpSocket, b: B) -> UdpStream<B> {
        UdpStream {
            socket: socket,
            pool: b,
        }
    }

    /// Return a reference to the underlying UDP socket
    pub fn socket<'a>(&'a self) -> &'a UdpSocket {
        &self.socket
    }
}

impl<B: BufferPool> Stream for UdpStream<B> {
    type Item = (B::Item, SocketAddr);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if let Async::NotReady = self.socket.poll_read() {
            return Ok(Async::NotReady)
        }
        let mut buf = try!(self.pool.get());
        let result = unsafe { self.socket.recv_from(buf.mut_bytes()) };
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


