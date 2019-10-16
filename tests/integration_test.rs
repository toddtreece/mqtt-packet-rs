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

#[test]
fn binary_data() {
    let data: Vec<u8> = vec![
        0x00, 0x01, 0x09, 0, 10, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A,
    ];
    let reader = io::BufReader::new(&*data);
    let property = Property::parse(reader);

    let expected: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
    match property.values.get("CorrelationData") {
        Some(value) => assert_eq!(value, &Type::BinaryData(expected)),
        None => panic!("Not a valid property"),
    }
}

#[test]
fn utf8_string() {
    let data: Vec<u8> = vec![
        0x00, 0x01, 0x1c, 0, 11, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 100, 100,
        100,
    ];
    let reader = io::BufReader::new(&*data);
    let property = Property::parse(reader);
    match property.values.get("ServerReference") {
        Some(value) => assert_eq!(value, &Type::Utf8EncodedString("hello world".to_string())),
        None => panic!("Not a valid property"),
    }
}

#[test]
fn utf8_string_pair() {
    let data: Vec<u8> = vec![
        0x00, 0x01, 0x26, 0, 11, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 0, 7, 102,
        111, 111, 32, 98, 97, 114, 1, 1, 1, 1,
    ];
    let reader = io::BufReader::new(&*data);
    let property = Property::parse(reader);
    match property.values.get("UserProperty") {
        Some(value) => assert_eq!(
            value,
            &Type::Utf8StringPair("hello world".to_string(), "foo bar".to_string())
        ),
        None => panic!("Not a valid property"),
    }
}
