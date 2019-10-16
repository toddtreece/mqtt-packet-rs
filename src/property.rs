use super::data_type::Type;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::io;

/**
 * 2.2.2.2 Property
 * https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901027
 * A Property consists of an Identifier which defines its usage and data type,
 * followed by a value. The Identifier is encoded as a Variable Byte Integer.
 * A Control Packet which contains an Identifier which is not valid for its
 * packet type, or contains a value not of the specified data type, is a
 * Malformed Packet. If received, use a CONNACK or DISCONNECT packet with
 * Reason Code 0x81 (Malformed Packet). There is no significance in the order
 * of Properties with different Identifiers.
 */
#[repr(u8)]
#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum Indentifier {
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
    SharedSubscriptionAvailable = 0x2a,
}

impl fmt::Display for Indentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub struct Property {
    pub values: HashMap<String, Type>,
}

impl Property {
    pub fn parse<R>(mut reader: R) -> Self
    where
        R: io::Read,
    {
        use Indentifier::*;
        let length = Type::parse_two_byte_int(&mut reader);
        let mut properties = HashMap::new();
        for _i in 0..length.into() {
            let mut id_buffer = [0; 1];
            reader.read(&mut id_buffer).expect("Reading error");
            let identifier = Indentifier::from_u8(id_buffer[0]).unwrap();
            let parsed = match identifier {
                PayloadFormatIndicator
                | RequestProblemInformation
                | RequestResponseInformation
                | MaximumQos
                | RetainAvailable
                | WildcardSubscriptionAvailable
                | SubscriptionIdentifierAvailable
                | SharedSubscriptionAvailable => Type::parse_byte(&mut reader),
                ServerKeepAlive | ReceiveMaximum | TopicAliasMaximum | TopicAlias => {
                    Type::parse_two_byte_int(&mut reader)
                }
                MessageExpiryInterval
                | SessionExpiryInterval
                | WillDelayInterval
                | MaximumPacketSize => Type::parse_four_byte_int(&mut reader),
                SubscriptionIdentifier => Type::parse_variable_byte_int(&mut reader),
                UserProperty => Type::parse_utf8_string_pair(&mut reader),
                CorrelationData | AuthenticationData => Type::parse_binary_data(&mut reader),
                ContentType
                | ResponseTopic
                | AssignedClientIdentifier
                | AuthenticationMethod
                | ResponseInformation
                | ServerReference
                | ReasonString => Type::parse_utf8_string(&mut reader),
            };
            properties.insert(identifier.to_string(), parsed);
        }

        return Self { values: properties };
    }
}
