use super::data_type::*;
use std::collections::HashMap;
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
struct Property(HashMap<String, Type>);

impl Property {
    pub fn parse<R, T>(mut reader: R) -> Self
    where
        R: io::Read,
    {
        let length = Type::parse_two_byte_int(&mut reader);
        let mut properties = HashMap::new();
        for _i in 0..length.into() {
            let mut id = [0; 1];
            reader.read(&mut id).expect("Reading error");
            match id[0] {
                0x01 => {
                    properties.insert(
                        "payloadFormatIndicator".to_string(),
                        Type::parse_byte(&mut reader),
                    );
                }
                0x02 => {
                    properties.insert(
                        "messageExpiryInterval".to_string(),
                        Type::parse_two_byte_int(&mut reader),
                    );
                }
                0x03 => {
                    properties.insert(
                        "contentType".to_string(),
                        Type::parse_utf8_string(&mut reader),
                    );
                }
                0x08 => {
                    properties.insert(
                        "responseTopic".to_string(),
                        Type::parse_utf8_string(&mut reader),
                    );
                }
                0x09 => {
                    properties.insert(
                        "correlationData".to_string(),
                        Type::parse_binary_data(&mut reader),
                    );
                }
                0x0b => {
                    properties.insert(
                        "subscriptionIdentifier".to_string(),
                        Type::parse_variable_byte_int(&mut reader),
                    );
                }
                0x11 => {
                    properties.insert(
                        "sessionExpiryInterval".to_string(),
                        Type::parse_four_byte_int(&mut reader),
                    );
                }
                0x12 => {
                    properties.insert(
                        "assignedClientIdentifier".to_string(),
                        Type::parse_utf8_string(&mut reader),
                    );
                }
                0x13 => {
                    properties.insert(
                        "serverKeepAlive".to_string(),
                        Type::parse_two_byte_int(&mut reader),
                    );
                }
                0x15 => {
                    properties.insert(
                        "authenticationMethod".to_string(),
                        Type::parse_utf8_string(&mut reader),
                    );
                }
                0x16 => {
                    properties.insert(
                        "authenticationData".to_string(),
                        Type::parse_binary_data(&mut reader),
                    );
                }
                0x17 => {
                    properties.insert(
                        "requestProblemInformation".to_string(),
                        Type::parse_byte(&mut reader),
                    );
                }
                0x18 => {
                    properties.insert(
                        "willDelayInterval".to_string(),
                        Type::parse_four_byte_int(&mut reader),
                    );
                }
                0x19 => {
                    properties.insert(
                        "requestResponseInformation".to_string(),
                        Type::parse_byte(&mut reader),
                    );
                }
                0x1a => {
                    properties.insert(
                        "responseInformation".to_string(),
                        Type::parse_utf8_string(&mut reader),
                    );
                }
                0x1c => {
                    properties.insert(
                        "serverReference".to_string(),
                        Type::parse_utf8_string(&mut reader),
                    );
                }
                0x1f => {
                    properties.insert(
                        "reasonString".to_string(),
                        Type::parse_utf8_string(&mut reader),
                    );
                }
                0x21 => {
                    properties.insert(
                        "receiveMaximum".to_string(),
                        Type::parse_two_byte_int(&mut reader),
                    );
                }
                0x22 => {
                    properties.insert(
                        "topicAliasMaximum".to_string(),
                        Type::parse_two_byte_int(&mut reader),
                    );
                }
                0x23 => {
                    properties.insert(
                        "topicAlias".to_string(),
                        Type::parse_utf8_string(&mut reader),
                    );
                }
                0x24 => {
                    properties.insert("maximumQos".to_string(), Type::parse_byte(&mut reader));
                }
                0x25 => {
                    properties.insert("retainAvailable".to_string(), Type::parse_byte(&mut reader));
                }
                0x26 => {
                    properties.insert(
                        "userProperty".to_string(),
                        Type::parse_utf8_string_pair(&mut reader),
                    );
                }
                0x27 => {
                    properties.insert(
                        "maximumPacketSize".to_string(),
                        Type::parse_four_byte_int(&mut reader),
                    );
                }
                0x28 => {
                    properties.insert(
                        "wildcardSubscriptionAvailable".to_string(),
                        Type::parse_byte(&mut reader),
                    );
                }
                0x29 => {
                    properties.insert(
                        "subscriptionIdentifierAvailable".to_string(),
                        Type::parse_byte(&mut reader),
                    );
                }
                0x2a => {
                    properties.insert(
                        "sharedSubscriptionAvailable".to_string(),
                        Type::parse_byte(&mut reader),
                    );
                }
                _ => println!("Invalid property"),
            }
        }

        return Self(properties);
    }
}
