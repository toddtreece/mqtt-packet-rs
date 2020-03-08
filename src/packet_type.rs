use crate::build_enum;
use crate::DataType;
use crate::Error;
use std::convert::TryFrom;

build_enum!(
  PacketType {
    CONNECT = 1,
    CONNACK = 2,
    PUBLISH = 3,
    PUBACK = 4,
    PUBREC = 5,
    PUBREL = 6,
    PUBCOMP = 7,
    SUBSCRIBE = 8,
    SUBACK = 9,
    UNSUBSCRIBE = 10,
    UNSUBACK = 11,
    PINGREQ = 12,
    PINGRESP = 13,
    DISCONNECT = 14,
    AUTH = 15
  }
);

/// [2.1.2 MQTT Control Packet type](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901022)
///
/// Position: byte 1, bits 7-4.
/// Represented as a 4-bit unsigned value.
impl PacketType {
  /// Parse property values from a reader into DataType variants.
  ///
  /// # Examples
  /// ```rust
  /// use mqtt_packet::PacketType;
  /// use mqtt_packet::DataType;
  /// use mqtt_packet::Error;
  /// use std::io;
  ///
  /// let bytes: Vec<u8> = vec![0x10];
  /// let mut reader = io::BufReader::new(&bytes[..]);
  /// let byte = DataType::parse_byte(&mut reader).unwrap();
  ///
  /// let packet_type = PacketType::new(byte).unwrap();
  /// assert_eq!(packet_type, PacketType::CONNECT);
  /// assert_eq!(packet_type.to_u8().unwrap(), 1);
  /// ```
  ///
  /// Error:
  ///
  /// ```rust
  /// use mqtt_packet::PacketType;
  /// use mqtt_packet::DataType;
  /// use mqtt_packet::Error;
  /// use std::io;
  ///
  /// let err_bytes: Vec<u8> = vec![0x00, 0x01];
  /// let mut err_reader = io::BufReader::new(&err_bytes[..]);
  /// let wrong_type = DataType::parse_two_byte_int(&mut err_reader).unwrap();
  ///
  /// let err = PacketType::new(wrong_type).unwrap_err();
  /// assert_eq!(err, Error::ParseError)
  /// ```
  pub fn new(byte: DataType) -> Result<Self, Error> {
    if let DataType::Byte(value) = byte {
      let type_number: u8 = (value & 0xF0) >> 4;
      PacketType::try_from(type_number)
    } else {
      Err(Error::ParseError)
    }
  }

  pub fn to_u8(self) -> Result<u8, Error> {
    Ok(u8::from(self))
  }
}

#[cfg(test)]
mod tests {
  use crate::DataType;
  use std::io;

  #[test]
  fn connect() {
    let bytes: Vec<u8> = vec![0x10];
    let mut reader = io::BufReader::new(&bytes[..]);
    let byte = DataType::parse_byte(&mut reader).unwrap();
    let packet_type = super::PacketType::new(byte).unwrap();
    assert_eq!(packet_type, super::PacketType::CONNECT);
    assert_eq!(packet_type.to_u8().unwrap(), 1);
  }

  #[test]
  fn auth() {
    let bytes: Vec<u8> = vec![0xF0];
    let mut reader = io::BufReader::new(&bytes[..]);
    let byte = DataType::parse_byte(&mut reader).unwrap();
    let packet_type = super::PacketType::new(byte).unwrap();
    assert_eq!(packet_type, super::PacketType::AUTH);
    assert_eq!(packet_type.to_u8().unwrap(), 15);
  }

  #[test]
  fn err_value() {
    let err_bytes: Vec<u8> = vec![0x00];
    let mut err_reader = io::BufReader::new(&err_bytes[..]);

    let byte = DataType::parse_byte(&mut err_reader).unwrap();
    let err = super::PacketType::new(byte).unwrap_err();
    assert_eq!(err, crate::Error::ParseError)
  }

  #[test]
  fn err_read() {
    let err_bytes: Vec<u8> = vec![0x00, 0x01];
    let mut err_reader = io::BufReader::new(&err_bytes[..]);
    let byte = DataType::parse_two_byte_int(&mut err_reader).unwrap();

    let err = super::PacketType::new(byte).unwrap_err();
    assert_eq!(err, crate::Error::ParseError)
  }
}
