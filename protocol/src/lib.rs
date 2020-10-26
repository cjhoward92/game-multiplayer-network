#[macro_use]
extern crate serde_derive;

pub use serde::{Serialize, Deserialize};

// The protocol id used to identify calls to this network
const PROTOCOL_ID: u32 = 0x10214B68;

// Flags used to define message types
pub mod flags {
    pub const HELLO: u8 = 0;
    pub const COMPLEX: u8 = 1;
    pub const TEST: u8 = 255;
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Hello {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Complex {
    pub hello: Hello,
    pub num: u32,
    pub is_true: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Test {
    pub num: u16,
    pub message: String,
}

// The full representation of a packet on the UDP Network
#[derive(Serialize, Deserialize)]
pub struct Packet {
    pub protocol: u32,
    pub flag: u8,
    pub len: u16,
    pub content: Box<Vec<u8>>,
}

fn verify_flag(flag: u8) {
    let found_flag = match flag {
        0..=1 => Ok(()),
        _ => Err(()),
    };
    found_flag.expect("Unknown flag");
}

pub fn from_packet<'a, T: Deserialize<'a>>(packet: &'a Packet) -> T {
    assert_eq!(packet.protocol, PROTOCOL_ID);
    assert_eq!(packet.len as usize, packet.content.len());
    verify_flag(packet.flag);

    bincode::deserialize(&packet.content[..]).unwrap()
}

pub fn to_packet<'a, T: Serialize>(flag: u8, data: &'a T) -> Packet {
    verify_flag(flag);

    let content = bincode::serialize(data).unwrap();

    Packet {
        protocol: PROTOCOL_ID,
        flag,
        len: content.len() as u16,
        content: Box::new(content),
    }
}

pub fn serialize_packet(packet: &Packet) -> Vec<u8> {
    bincode::serialize(packet).unwrap()
}

pub fn deserialize_packet(data: &[u8]) -> Packet {
    bincode::deserialize(data).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{Test, Packet, PROTOCOL_ID, from_packet};
    use super::flags::TEST;

    #[test]
    fn test_decode_as_test_struct() {
        let test = Test {
            num: 100,
            message: String::from("Hello there"),
        };

        let bytes = bincode::serialize(&test).unwrap();
        for b in &bytes {
            print!("{:#010b} ", b);
        }
        println!("");
        let packet = Packet {
            protocol: PROTOCOL_ID,
            flag: TEST,
            len: bytes.len() as u16,
            content: Box::new(bytes),
        };

        let rebuilt_test: Test = from_packet(&packet);
        assert_eq!(rebuilt_test.num, test.num);
        assert_eq!(rebuilt_test.message, test.message);
    }
}
