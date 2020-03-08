use crate::build_enum;
use crate::DataType;
use crate::Error;
use std::convert::TryFrom;

build_enum!(
  ReasonCode {
    SUCCESS = 0x00,
    GRANTED_QOS_1 = 0x01,
    GRANTED_QOS_2 = 0x02,
    DISCONNECT_WITH_WILL_MESSAGE = 0x04,
    NO_MATCHING_SUBSCRIBERS = 0x10,
    NO_SUBSCRIPTION_EXISTED = 0x11,
    CONTINUE_AUTHENTICATION = 0x18,
    RE_AUTHENTICATE = 0x19,
    UNSPECIFIED_ERROR = 0x80,
    MALFORMED_PACKET = 0x81,
    PROTOCOL_ERROR = 0x82,
    IMPLEMENTATION_SPECIFIC_ERROR = 0x83,
    UNSUPPORTED_PROTOCOL_VERSION = 0x84,
    CLIENT_IDENTIFIER_NOT_VALID = 0x85,
    BAD_USER_NAME_OR_PASSWORD = 0x86,
    NOT_AUTHORIZED = 0x87,
    SERVER_UNAVAILABLE = 0x88,
    SERVER_BUSY = 0x89,
    BANNED = 0x8A,
    SERVER_SHUTTING_DOWN = 0x8B,
    BAD_AUTHENTICATION_METHOD = 0x8C,
    KEEP_ALIVE_TIMEOUT = 0x8D,
    SESSION_TAKEN_OVER = 0x8E,
    TOPIC_FILTER_INVALID = 0x8F,
    TOPIC_NAME_INVALID = 0x90,
    PACKET_IDENTIFIER_IN_USE = 0x91,
    PACKET_IDENTIFIER_NOT_FOUND = 0x92,
    RECEIVE_MAXIMUM_EXCEEDED = 0x93,
    TOPIC_ALIAS_INVALID = 0x94,
    PACKET_TOO_LARGE = 0x95,
    MESSAGE_RATE_TOO_HIGH = 0x96,
    QUOTA_EXCEEDED = 0x97,
    ADMINISTRATIVE_ACTION = 0x98,
    PAYLOAD_FORMAT_INVALID = 0x99,
    RETAIN_NOT_SUPPORTED = 0x9A,
    QOS_NOT_SUPPORTED = 0x9B,
    USE_ANOTHER_SERVER = 0x9C,
    SERVER_MOVED = 0x9D,
    SHARED_SUBSCRIPTIONS_NOT_SUPPORTED = 0x9E,
    CONNECTION_RATE_EXCEEDED = 0x9F,
    MAXIMUM_CONNECT_TIME = 0xA0,
    SUBSCRIPTION_IDENTIFIERS_NOT_SUPPORTED = 0xA1,
    WILDCARD_SUBSCRIPTIONS_NOT_SUPPORTED = 0xA2
  }
);

/// [2.4 Reason Code](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901031)
///
/// A Reason Code is a one byte unsigned value that indicates the result of an
/// operation. Reason Codes less than 0x80 indicate successful completion of an
/// operation. The normal Reason Code for success is 0. Reason Code values of
/// 0x80 or greater indicate failure.
///
/// The CONNACK, PUBACK, PUBREC, PUBREL, PUBCOMP, DISCONNECT and AUTH Control
/// Packets have a single Reason Code as part of the Variable Header. The SUBACK
/// and UNSUBACK packets contain a list of one or more Reason Codes in the Payload.
impl ReasonCode {
  /// # Examples
  /// ```rust
  /// use mqtt_packet::ReasonCode;
  /// use mqtt_packet::DataType;
  /// use mqtt_packet::Error;
  /// use std::io;
  ///
  /// let bytes: Vec<u8> = vec![0x10];
  /// let mut reader = io::BufReader::new(&bytes[..]);
  /// let byte = DataType::parse_byte(&mut reader).unwrap();
  ///
  /// let reason_code = ReasonCode::new(byte).unwrap();
  /// assert_eq!(reason_code, ReasonCode::NO_MATCHING_SUBSCRIBERS);
  /// assert_eq!(reason_code.to_u8().unwrap(), 0x10);
  /// ```
  ///
  /// Error:
  ///
  /// ```rust
  /// use mqtt_packet::ReasonCode;
  /// use mqtt_packet::DataType;
  /// use mqtt_packet::Error;
  /// use std::io;
  ///
  /// let err_bytes: Vec<u8> = vec![0x02, 0x01];
  /// let mut err_reader = io::BufReader::new(&err_bytes[..]);
  /// let wrong_type = DataType::parse_two_byte_int(&mut err_reader).unwrap();
  ///
  /// let err = ReasonCode::new(wrong_type).unwrap_err();
  /// assert_eq!(err, Error::ParseError)
  /// ```
  pub fn new(byte: DataType) -> Result<Self, Error> {
    if let DataType::Byte(value) = byte {
      ReasonCode::try_from(value)
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
  fn no_matching_subscribers() {
    let bytes: Vec<u8> = vec![0x10];
    let mut reader = io::BufReader::new(&bytes[..]);
    let byte = DataType::parse_byte(&mut reader).unwrap();
    let reason_code = super::ReasonCode::new(byte).unwrap();
    assert_eq!(reason_code, super::ReasonCode::NO_MATCHING_SUBSCRIBERS);
    assert_eq!(reason_code.to_u8().unwrap(), 0x10);
  }

  #[test]
  fn wildcard_subscriptions_not_supported() {
    let bytes: Vec<u8> = vec![0xA2];
    let mut reader = io::BufReader::new(&bytes[..]);
    let byte = DataType::parse_byte(&mut reader).unwrap();
    let reason_code = super::ReasonCode::new(byte).unwrap();
    assert_eq!(
      reason_code,
      super::ReasonCode::WILDCARD_SUBSCRIPTIONS_NOT_SUPPORTED
    );
    assert_eq!(reason_code.to_u8().unwrap(), 0xA2);
  }

  #[test]
  fn err_value() {
    let err_bytes: Vec<u8> = vec![0x03];
    let mut err_reader = io::BufReader::new(&err_bytes[..]);

    let byte = DataType::parse_byte(&mut err_reader).unwrap();
    let err = super::ReasonCode::new(byte).unwrap_err();
    assert_eq!(err, crate::Error::ParseError)
  }

  #[test]
  fn err_read() {
    let err_bytes: Vec<u8> = vec![0x02, 0x01];
    let mut err_reader = io::BufReader::new(&err_bytes[..]);
    let byte = DataType::parse_two_byte_int(&mut err_reader).unwrap();

    let err = super::ReasonCode::new(byte).unwrap_err();
    assert_eq!(err, crate::Error::ParseError)
  }
}
