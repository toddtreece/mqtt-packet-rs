use mqtt_packet::data_type::Type;
use mqtt_packet::property::Property;
use std::io;

#[test]
fn byte() {
    let reader: Vec<u8> = vec![0x00, 0x02, 0x01, 0xFF, 0x24, 0x02];
    let property = Property::parse(&*reader);

    match property.values.get("PayloadFormatIndicator") {
        Some(value) => assert_eq!(value, &Type::Byte(255)),
        None => panic!("Not a valid property"),
    }

    match property.values.get("MaximumQos") {
        Some(value) => assert_eq!(value, &Type::Byte(2)),
        None => panic!("Not a valid property"),
    }
}

#[test]
fn two_byte() {
    let reader: Vec<u8> = vec![0x00, 0x01, 0x13, 0x02, 0x03];
    let property = Property::parse(&*reader);
    match property.values.get("ServerKeepAlive") {
        Some(value) => assert_eq!(value, &Type::TwoByteInteger(515)),
        None => panic!("Not a valid property"),
    }
}

#[test]
fn four_byte() {
    let reader: Vec<u8> = vec![0x00, 0x01, 0x02, 0x02, 0x03, 0x04, 0x05];
    let property = Property::parse(&*reader);
    match property.values.get("MessageExpiryInterval") {
        Some(value) => assert_eq!(value, &Type::FourByteInteger(33752069)),
        None => panic!("Not a valid property"),
    }
}

#[test]
fn variable_byte() {
    let reader: Vec<u8> = vec![0x00, 0x01, 0x0b, 0xFF, 0xFF, 0xFF, 0x7F];
    let property = Property::parse(&*reader);
    match property.values.get("SubscriptionIdentifier") {
        Some(value) => assert_eq!(value, &Type::FourByteInteger(268435455)),
        None => panic!("Not a valid property"),
    }
}
