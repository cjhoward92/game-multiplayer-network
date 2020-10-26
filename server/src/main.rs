use std::net::UdpSocket;
use protocol::{from_packet, deserialize_packet, Hello, Complex};

fn run() -> std::io::Result<()> {
    println!("Starting server");
    let socket = UdpSocket::bind("127.0.0.1:10000")?;

    let mut buf = [0; 1024];
    let (amt, src) = socket.recv_from(&mut buf)?;
    println!("Found {} bytes", amt);

    let packet = deserialize_packet(&buf[0..amt]);
    let hello: Hello = from_packet(&packet);

    println!("Received \"{}\" from {}", hello.message, src);

    // Test 2
    let mut buf = [0; 1024];
    let (amt, _src) = socket.recv_from(&mut buf)?;
    println!("Found {} bytes", amt);

    let packet = deserialize_packet(&buf[0..amt]);
    let complex: Complex = from_packet(&packet);

    println!("Message: {}, Num: {}, Is True: {}", complex.hello.message, complex.num, complex.is_true);

    // Redeclare `buf` as slice of the received data and send reverse data back to origin.
    // let buf = &mut buf[..amt];
    // buf.reverse();
    // socket.send_to(buf, &src)?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    run()
}
