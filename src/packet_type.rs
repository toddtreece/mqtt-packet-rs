use crate::data_type::DataType;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::io;

/**
 * 2.1.2 MQTT Control Packet type
 * https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901022
 * Position: byte 1, bits 7-4.
 * Represented as a 4-bit unsigned value.
 */
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, FromPrimitive, ToPrimitive)]
pub enum PacketType {
  CONNECT = 1,
  CONNACK,
  PUBLISH,
  PUBACK,
  PUBREC,
  PUBREL,
  PUBCOMP,
  SUBSCRIBE,
  SUBACK,
  UNSUBSCRIBE,
  UNSUBACK,
  PINGREQ,
  PINGRESP,
  DISCONNECT,
  AUTH,
}

impl PacketType {
  /**
   * Parse property values from a reader into DataType variants.
   */
  pub fn new<R>(mut reader: R) -> Self
  where
    R: io::Read,
  {
    let byte = DataType::parse_byte(&mut reader);
    if let DataType::Byte(value) = byte {
      let type_number: u8 = (value & 0xF0) >> 4;
      return PacketType::from_u8(type_number).unwrap();
    } else {
      panic!("Unknown control packet type");
    }
  }
}

mod tests {
  #[test]
  fn connect() {
    let reader: Vec<u8> = vec![0x10];
    let packet_type = super::PacketType::new(&*reader);
    assert_eq!(packet_type, super::PacketType::CONNECT);
  }

  #[test]
  fn auth() {
    let reader: Vec<u8> = vec![0xF0];
    let packet_type = super::PacketType::new(&*reader);
    assert_eq!(packet_type, super::PacketType::AUTH);
  }

  #[test]
  #[should_panic]
  fn packet_type_panic() {
    let reader: Vec<u8> = vec![0x00];
    let _packet_type = super::PacketType::new(&*reader);
  }
}
