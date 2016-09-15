
use std::io;
use std::net::SocketAddr;

use net::{ UdpSocket, BufferPool };
use futures::{Async, Poll};
use futures::stream::Stream;

///
/// UdpStream
/// Wraps the UdpSocket and provides a `futures::stream::Stream` implementation
///
pub struct UdpStream<B : BufferPool> {
    socket: UdpSocket,
    pool: B
}

impl<B : BufferPool> UdpStream<B> {
    /// Creates a new UdpStream.  The Buffer pull is a factory of fixed sized
    /// buffers which is leveraged so that the UdpStream may continually produce
    /// data.
    pub fn new(socket: UdpSocket, b: B) -> UdpStream<B> {
        UdpStream {
            socket: socket,
            pool: b
        }
    }
}

impl<B: BufferPool> Stream for UdpStream<B> {
    type Item = (Vec<u8>, SocketAddr);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if let Async::NotReady = self.socket.poll_read() {
            return Ok(Async::NotReady)
        }
        let mut buf = try!(self.pool.get());
        match self.socket.recv_from(buf.as_mut_slice()) {
            Ok((amt, addr)) => { 
                unsafe { buf.set_len(amt); };
                Ok(Async::Ready(Some((buf, addr)))) },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                Ok(Async::NotReady)
            },
            Err(e) => Err(e)
        }
    }
}
