
use std::io;
use std::net::SocketAddr;

use net::UdpSocket;
use net::{ Buffer, BufferPool };
use io::Io;
use futures::{Future, Async, Poll};
use futures::stream::Stream;

pub struct UdpStream<B : BufferPool> {
    socket: UdpSocket,
    pool: B
}

impl<B: BufferPool> UdpStream<B> {
    pub fn new(socket: UdpSocket, b: B) -> UdpStream<B> {
        UdpStream {
            socket: socket,
            pool: b
        }
    }
}

impl<B: BufferPool> Stream for UdpStream<B> {
    type Item = (Buffer + Sized, SocketAddr);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if let Async::NotReady = self.socket.poll_read() {
            return Ok(Async::NotReady)
        }
        let buf = try!(self.pool.get());
        match self.socket.recv_from(buf.as_mut()) {
            Ok((amt, addr)) => unsafe { 
                buf.advance(amt);
                Ok(Async::Ready(Some((buf[..amt], addr)))) },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                self.inner.io.need_read();
                Ok(Async::NotReady)
            }
        }
    }
}
