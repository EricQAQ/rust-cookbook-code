/// Read and write integers in little-endian byte order
#[macro_use]
extern crate error_chain;
extern crate byteorder;

use std::io;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

error_chain! {
    foreign_links {
        ConvertError(io::Error);
    }
}

#[derive(Default, PartialEq, Debug)]
struct Payload {
    kind: u8,
    value: u16,
}

fn encode(payload: &Payload) -> Result<Vec<u8>> {
    let mut bytes = vec![];
    bytes.write_u8(payload.kind)?;
    bytes.write_u16::<LittleEndian>(payload.value)?;
    Ok(bytes)
}

fn decode(mut bytes: &[u8]) -> Result<Payload> {
    let payload = Payload {
        kind: bytes.read_u8()?,
        value: bytes.read_u16::<LittleEndian>()?,
    };
    Ok(payload)
}

fn run() -> Result<()> {
    let original_payload = Payload::default();
    let encoded_bytes = encode(&original_payload)?;
    let decode_bytes = decode(&encoded_bytes)?;
    assert_eq!(original_payload, decode_bytes);
    Ok(())
}

quick_main!(run);
