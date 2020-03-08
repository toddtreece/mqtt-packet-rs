use crate::build_enum;
use crate::DataType;
use crate::Error;
use std::convert::TryFrom;

build_enum!(
  ReasonCode {
    Success = 0x00,
    GrantedQos1 = 0x01,
    GrantedQos2 = 0x02,
    DisconnectWithWillMessage = 0x04,
    NoMatchingSubscribers = 0x10,
    NoSubscriptionExisted = 0x11,
    ContinueAuthentication = 0x18,
    ReAuthenticate = 0x19,
    UnspecifiedError = 0x80,
    MalformedPacket = 0x81,
    ProtocolError = 0x82,
    ImplementationSpecificError = 0x83,
    UnsupportedProtocolVersion = 0x84,
    ClientIdentifierNotValid = 0x85,
    BadUserNameOrPassword = 0x86,
    NotAuthorized = 0x87,
    ServerUnavailable = 0x88,
    ServerBusy = 0x89,
    Banned = 0x8A,
    ServerShuttingDown = 0x8B,
    BadAuthenticationMethod = 0x8C,
    KeepAliveTimeout = 0x8D,
    SessionTakenOver = 0x8E,
    TopicFilterInvalid = 0x8F,
    TopicNameInvalid = 0x90,
    PacketIdentifierInUse = 0x91,
    PacketIdentifierNotFound = 0x92,
    ReceiveMaximumExceeded = 0x93,
    TopicAliasInvalid = 0x94,
    PacketTooLarge = 0x95,
    MessageRateTooHigh = 0x96,
    QuotaExceeded = 0x97,
    AdministrativeAction = 0x98,
    PayloadFormatInvalid = 0x99,
    RetainNotSupported = 0x9A,
    QosNotSupported = 0x9B,
    UseAnotherServer = 0x9C,
    ServerMoved = 0x9D,
    SharedSubscriptionsNotSupported = 0x9E,
    ConnectionRateExceeded = 0x9F,
    MaximumConnectTime = 0xA0,
    SubscriptionIdentifiersNotSupported = 0xA1,
    WildcardSubscriptionsNotSupported = 0xA2
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
  /// assert_eq!(reason_code, ReasonCode::NoMatchingSubscribers);
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
    assert_eq!(reason_code, super::ReasonCode::NoMatchingSubscribers);
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
      super::ReasonCode::WildcardSubscriptionsNotSupported
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
