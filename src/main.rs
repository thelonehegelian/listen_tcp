use std::io::{Error, Read};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
//  the program will listen until a request is made
fn main() -> Result<(), Error> {
    let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0);
    let listener = TcpListener::bind(addr)?;
    // assigns random port
    let port = listener.local_addr()?;
    println!("{}", port);
    println!("Listening on port {}", port.port());
    // accept a single connection, creates a new TcpStream
    let (mut stream, addr) = listener.accept()?;
    println!("Connection received from {}", addr);
    // read incoming to string
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;
    // print the incoming message
    println!("Message received: {}", buffer);
    Ok(())
}
