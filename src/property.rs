use crate::build_enum;
use crate::DataType;
use crate::Error;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::io;

build_enum!(PropertyIdentifier {
  PayloadFormatIndicator = 0x01,
  MessageExpiryInterval = 0x02,
  ContentType = 0x03,
  ResponseTopic = 0x08,
  CorrelationData = 0x09,
  SubscriptionIdentifier = 0x0b,
  SessionExpiryInterval = 0x11,
  AssignedClientIdentifier = 0x12,
  ServerKeepAlive = 0x13,
  AuthenticationMethod = 0x15,
  AuthenticationData = 0x16,
  RequestProblemInformation = 0x17,
  WillDelayInterval = 0x18,
  RequestResponseInformation = 0x19,
  ResponseInformation = 0x1a,
  ServerReference = 0x1c,
  ReasonString = 0x1f,
  ReceiveMaximum = 0x21,
  TopicAliasMaximum = 0x22,
  TopicAlias = 0x23,
  MaximumQos = 0x24,
  RetainAvailable = 0x25,
  UserProperty = 0x26,
  MaximumPacketSize = 0x27,
  WildcardSubscriptionAvailable = 0x28,
  SubscriptionIdentifierAvailable = 0x29,
  SharedSubscriptionAvailable = 0x2a
});

/// A Property consists of an Identifier which defines its usage and data type,
/// followed by a value.
///
/// # [2.2.2.2 Property](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901027)
///
/// A Property consists of an Identifier which defines its usage and data type,
/// followed by a value. The Identifier is encoded as a Variable Byte Integer.
/// A Control Packet which contains an Identifier which is not valid for its
/// packet type, or contains a value not of the specified data type, is a
/// Malformed Packet. If received, use a CONNACK or DISCONNECT packet with
/// Reason Code 0x81 (Malformed Packet). There is no significance in the order
/// of Properties with different Identifiers.
pub struct Property {
  pub values: BTreeMap<PropertyIdentifier, DataType>,
}

impl Property {
  /// Parse property identifiers and values from a reader.
  pub fn new<R: io::Read>(reader: &mut R) -> Result<Self, Error> {
    let mut length: u16 = DataType::parse_two_byte_int(reader)?.into();
    let mut properties = BTreeMap::new();

    while length > 0 {
      let identifier = Self::parse_identifier(reader)?;
      length -= 1;

      let data_type = Self::parse_type(identifier, reader)?;
      let data_length = data_type.byte_len()?;

      // something is wrong if the total length of properties doesn't match
      if data_length > length {
        return Err(Error::MalformedPacket);
      } else {
        length -= data_type.byte_len()?;
      }

      properties.insert(identifier, data_type);
    }

    Ok(Self { values: properties })
  }

  /// Parse Identifier variant from reader.
  fn parse_identifier<R: io::Read>(reader: &mut R) -> Result<PropertyIdentifier, Error> {
    let mut id_buffer = [0; 1];
    reader.read_exact(&mut id_buffer)?;
    Ok(PropertyIdentifier::try_from(id_buffer[0])?)
  }

  /// Parse property values from a reader into DataType variants.
  fn parse_type<R: io::Read>(
    identifier: PropertyIdentifier,
    reader: &mut R,
  ) -> Result<DataType, Error> {
    use PropertyIdentifier::*;

    match identifier {
      PayloadFormatIndicator
      | RequestProblemInformation
      | RequestResponseInformation
      | MaximumQos
      | RetainAvailable
      | WildcardSubscriptionAvailable
      | SubscriptionIdentifierAvailable
      | SharedSubscriptionAvailable => DataType::parse_byte(reader),
      ServerKeepAlive | ReceiveMaximum | TopicAliasMaximum | TopicAlias => {
        DataType::parse_two_byte_int(reader)
      }
      MessageExpiryInterval | SessionExpiryInterval | WillDelayInterval | MaximumPacketSize => {
        DataType::parse_four_byte_int(reader)
      }
      SubscriptionIdentifier => DataType::parse_variable_byte_int(reader),
      UserProperty => DataType::parse_utf8_string_pair(reader),
      CorrelationData | AuthenticationData => DataType::parse_binary_data(reader),
      ContentType
      | ResponseTopic
      | AssignedClientIdentifier
      | AuthenticationMethod
      | ResponseInformation
      | ServerReference
      | ReasonString => DataType::parse_utf8_string(reader),
    }
  }

  /// Convert Property values into a byte vector.
  pub fn generate(&self) -> Result<Vec<u8>, Error> {
    // create a vector to hold the generated data
    let mut props = vec![];

    // PartialOrd sorts enum variants in the order they are declared.
    for (key, value) in self.values.iter() {
      let id: u8 = u8::from(*key);
      props.push(vec![id]);
      props.push(value.to_vec()?);
    }

    let bytes = props.concat();

    // we need to fit the usize into a u16, so we can grab the first two bytes
    let length = u16::try_from(bytes.len() & 0xFFFF)
      .unwrap()
      .to_be_bytes()
      .to_vec();

    let result = vec![length, bytes];

    Ok(result.concat())
  }
}
