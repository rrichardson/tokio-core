
extern crate futures;
extern crate bytes;
extern crate tokio_core;
extern crate tokio_timer;

use std::net::ToSocketAddrs;
use std::str;
use futures::Future;
use futures::stream::Stream;
use tokio_core::reactor::Core;
use tokio_core::net::stream::Udp as UdpStream;
use tokio_core::net::{ UdpSocket, ByteBufPool }; 
use tokio_timer::Timer;
use std::time::Duration;
use std::sync::Arc;

fn main() {
    let mut core = Core::new().unwrap();
    let srvaddr = "127.0.0.1:9999".to_socket_addrs().unwrap().next().unwrap();
    let cliaddr = "127.0.0.1:9998".to_socket_addrs().unwrap().next().unwrap();

    let clipool = ByteBufPool::new(1024 * 8);
    let srvpool = ByteBufPool::new(1024 * 8);

    let srv = Arc::new(UdpSocket::bind(&srvaddr, &core.handle()).unwrap());
    let cli = Arc::new(UdpSocket::bind(&cliaddr, &core.handle()).unwrap());
   
    let srv2 = srv.clone();
    let cli2 = cli.clone();

    let srvstream = UdpStream::new(srv, srvpool);
    let clistream = UdpStream::new(cli, clipool);
    
    let app = cli2.send_dgram(b"PING", &srvaddr).and_then(|_| {
        let server = srvstream.for_each(|(buf, addr)| { 
            println!("{}", str::from_utf8(buf.bytes()).unwrap());
            srv2.send_dgram(b"PONG", &addr).map(|_| ()).wait()
        });
        let client = clistream.for_each(|(buf, addr)| { 
            println!("{}", str::from_utf8(buf.bytes()).unwrap());
            cli2.send_dgram(b"PING", &addr).map(|_| ()).wait()
        });
        server.join(client)
    });

    let timer = Timer::default(); 
    let wait = timer.timeout(app, Duration::from_millis(500));

    core.run(wait).unwrap();
}

