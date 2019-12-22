use crate::build_enum;
use crate::DataType;
use crate::Error;
use std::convert::TryFrom;
use std::io;

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
  /// use mqtt_packet::Error;
  /// use std::io;
  ///
  /// let bytes: Vec<u8> = vec![0x10];
  /// let mut reader = io::BufReader::new(&bytes[..]);
  ///
  /// let packet_type = PacketType::new(&mut reader).unwrap();
  /// assert_eq!(packet_type, PacketType::CONNECT);
  /// ```
  ///
  /// Error:
  ///
  /// ```rust
  /// use mqtt_packet::PacketType;
  /// use mqtt_packet::Error;
  /// use std::io;
  ///
  /// let err_bytes: Vec<u8> = vec![0x00];
  /// let mut err_reader = io::BufReader::new(&err_bytes[..]);
  ///
  /// let err = PacketType::new(&mut err_reader).unwrap_err();
  /// assert_eq!(err, Error::GenerateError)
  /// ```
  pub fn new<R: io::Read>(reader: &mut R) -> Result<Self, Error> {
    let byte = DataType::parse_byte(reader)?;
    if let DataType::Byte(value) = byte {
      let type_number: u8 = (value & 0xF0) >> 4;
      return Ok(PacketType::try_from(type_number)?);
    } else {
      return Err(Error::ParseError);
    }
  }
}

#[cfg(test)]
mod tests {
  use std::io;

  #[test]
  fn connect() {
    let bytes: Vec<u8> = vec![0x10];
    let mut reader = io::BufReader::new(&bytes[..]);
    let packet_type = super::PacketType::new(&mut reader);
    assert_eq!(packet_type.unwrap(), super::PacketType::CONNECT);
  }

  #[test]
  fn auth() {
    let bytes: Vec<u8> = vec![0xF0];
    let mut reader = io::BufReader::new(&bytes[..]);
    let packet_type = super::PacketType::new(&mut reader);
    assert_eq!(packet_type.unwrap(), super::PacketType::AUTH);
  }

  #[test]
  fn err() {
    let err_bytes: Vec<u8> = vec![0x00];
    let mut err_reader = io::BufReader::new(&err_bytes[..]);

    let err = super::PacketType::new(&mut err_reader).unwrap_err();
    assert_eq!(err, crate::Error::GenerateError)
  }
}
