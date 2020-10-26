use std::net::UdpSocket;
use protocol::{to_packet, serialize_packet, Hello, Complex};
use protocol::flags::{HELLO, COMPLEX};

fn run() -> std::io::Result<()> {
    println!("Starting client");
    let socket = UdpSocket::bind("127.0.0.1:10001")?;

    socket.connect("127.0.0.1:10000")?;

    let packet = to_packet(HELLO, &(Hello {
        message: String::from("Hello darkness my old friend"),
    }));
    
    let data = serialize_packet(&packet);
    println!("Sending content of size {}", data.len());
    socket.send(&data).unwrap();

    let packet = to_packet(COMPLEX, &(Complex {
        hello: Hello {
            message: String::from("Yikes! This is complicated!"),
        },
        num: 1890,
        is_true: false,
    }));
    
    let data = serialize_packet(&packet);
    println!("Sending content of size {}", data.len());
    socket.send(&data).unwrap();

    // let mut buf = [0; 10];
    // let (amt, src) = socket.recv_from(&mut buf)?;

    // println!("Received \"{}\" from {}", String::from_utf8_lossy(&buf), src);

    // // Redeclare `buf` as slice of the received data and send reverse data back to origin.
    // let buf = &mut buf[..amt];
    // buf.reverse();
    // socket.send_to(buf, &src)?;

    Ok(())
}

fn main() {
    run().expect("Something went wrong")
}
