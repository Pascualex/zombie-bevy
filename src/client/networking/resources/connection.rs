use std::net::{TcpStream, ToSocketAddrs};

pub struct Connection {
    pub stream: TcpStream,
}

impl Connection {
    pub fn new<A: ToSocketAddrs>(address: A) -> Self {
        let stream = TcpStream::connect(address).unwrap();
        stream.set_nonblocking(true).unwrap();
        Self { stream }
    }
}
