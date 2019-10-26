use mqtt_packet::control_packet::property::{Identifier::*, Property};
use mqtt_packet::data_type::{DataType, VariableByte};
use std::collections::BTreeMap;
use std::io;

#[test]
fn parse_byte() {
    let reader: Vec<u8> = vec![0x00, 0x02, 0x01, 0xFF, 0x24, 0x02];
    let property = Property::parse(&*reader);

    match property.values.get(&PayloadFormatIndicator) {
        Some(value) => assert_eq!(value, &DataType::Byte(255)),
        None => panic!("Not a valid property"),
    }

    match property.values.get(&MaximumQos) {
        Some(value) => assert_eq!(value, &DataType::Byte(2)),
        None => panic!("Not a valid property"),
    }
}

#[test]
fn parse_two_byte() {
    let reader: Vec<u8> = vec![0x00, 0x01, 0x13, 0x02, 0x03];
    let property = Property::parse(&*reader);
    match property.values.get(&ServerKeepAlive) {
        Some(value) => assert_eq!(value, &DataType::TwoByteInteger(515)),
        None => panic!("Not a valid property"),
    }
}

#[test]
fn parse_four_byte() {
    let reader: Vec<u8> = vec![0x00, 0x01, 0x02, 0x02, 0x03, 0x04, 0x05];
    let property = Property::parse(&*reader);
    match property.values.get(&MessageExpiryInterval) {
        Some(value) => assert_eq!(value, &DataType::FourByteInteger(33752069)),
        None => panic!("Not a valid property"),
    }
}

#[test]
fn parse_variable_byte() {
    let reader: Vec<u8> = vec![0x00, 0x01, 0x0b, 0xFF, 0xFF, 0xFF, 0x7F];
    let property = Property::parse(&*reader);
    match property.values.get(&SubscriptionIdentifier) {
        Some(value) => assert_eq!(
            value,
            &DataType::VariableByteInteger(VariableByte::Four(268435455))
        ),
        None => panic!("Not a valid property"),
    }
}

#[test]
fn parse_binary_data() {
    let data: Vec<u8> = vec![
        0x00, 0x01, 0x09, 0, 10, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A,
    ];
    let reader = io::BufReader::new(&*data);
    let property = Property::parse(reader);

    let expected: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
    match property.values.get(&CorrelationData) {
        Some(value) => assert_eq!(value, &DataType::BinaryData(expected)),
        None => panic!("Not a valid property"),
    }
}

#[test]
fn parse_utf8_string() {
    let data: Vec<u8> = vec![
        0x00, 0x01, 0x1c, 0, 11, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 100, 100,
        100,
    ];
    let reader = io::BufReader::new(&*data);
    let property = Property::parse(reader);
    match property.values.get(&ServerReference) {
        Some(value) => assert_eq!(
            value,
            &DataType::Utf8EncodedString("hello world".to_string())
        ),
        None => panic!("Not a valid property"),
    }
}

#[test]
fn parse_utf8_string_pair() {
    let data: Vec<u8> = vec![
        0x00, 0x01, 0x26, 0, 11, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 0, 7, 102,
        111, 111, 32, 98, 97, 114, 1, 1, 1, 1,
    ];
    let reader = io::BufReader::new(&*data);
    let property = Property::parse(reader);
    match property.values.get(&UserProperty) {
        Some(value) => assert_eq!(
            value,
            &DataType::Utf8StringPair("hello world".to_string(), "foo bar".to_string())
        ),
        None => panic!("Not a valid property"),
    }
}

fn all_data() -> Vec<u8> {
    let length: Vec<u8> = vec![0x00, 0x07];

    let byte: Vec<u8> = vec![0x01, 0xFF];
    let two_byte: Vec<u8> = vec![0x13, 0x02, 0x03];
    let four_byte: Vec<u8> = vec![0x02, 0x02, 0x03, 0x04, 0x05];
    let variable_byte: Vec<u8> = vec![0x0b, 0xFF, 0xFF, 0xFF, 0x7F];

    let binary_data: Vec<u8> = vec![
        0x09, 0, 10, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
    ];

    let string: Vec<u8> = vec![
        0x1c, 0, 11, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100,
    ];

    let string_pair: Vec<u8> = vec![
        0x26, 0, 11, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 0, 7, 102, 111, 111, 32,
        98, 97, 114,
    ];

    // these are sorted by the identifier keys used in
    // parse_all and generate_all. PartialOrd sorts enum
    // variants in the order they are declared.
    return [
        &length[..],
        &byte[..],
        &four_byte[..],
        &binary_data[..],
        &variable_byte[..],
        &two_byte[..],
        &string[..],
        &string_pair[..],
    ]
    .concat();
}

#[test]
fn parse_all() {
    let data = all_data();
    let reader = io::BufReader::new(&*data);
    let property = Property::parse(reader);

    for (identifier, value) in &property.values {
        match identifier {
            PayloadFormatIndicator => assert_eq!(value, &DataType::Byte(255)),
            ServerKeepAlive => assert_eq!(value, &DataType::TwoByteInteger(515)),
            MessageExpiryInterval => assert_eq!(value, &DataType::FourByteInteger(33752069)),
            SubscriptionIdentifier => assert_eq!(
                value,
                &DataType::VariableByteInteger(VariableByte::Four(268435455))
            ),
            CorrelationData => assert_eq!(
                value,
                &DataType::BinaryData(vec![
                    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09
                ])
            ),
            ServerReference => assert_eq!(
                value,
                &DataType::Utf8EncodedString("hello world".to_string())
            ),
            UserProperty => assert_eq!(
                value,
                &DataType::Utf8StringPair("hello world".to_string(), "foo bar".to_string())
            ),
            _ => panic!("Not a valid property"),
        }
    }
}

#[test]
fn generate_byte() {
    let mut property = Property {
        values: BTreeMap::new(),
    };

    property
        .values
        .insert(PayloadFormatIndicator, DataType::Byte(255));

    property.values.insert(MaximumQos, DataType::Byte(2));

    let expected: Vec<u8> = vec![0x00, 0x02, 0x01, 0xFF, 0x24, 0x02];
    assert_eq!(property.generate(), expected);
}

#[test]
fn generate_two_byte() {
    let mut property = Property {
        values: BTreeMap::new(),
    };

    property
        .values
        .insert(ServerKeepAlive, DataType::TwoByteInteger(515));

    let expected: Vec<u8> = vec![0x00, 0x01, 0x13, 0x02, 0x03];
    assert_eq!(property.generate(), expected);
}

#[test]
fn generate_four_byte() {
    let mut property = Property {
        values: BTreeMap::new(),
    };

    property
        .values
        .insert(MessageExpiryInterval, DataType::FourByteInteger(33752069));

    let expected: Vec<u8> = vec![0x00, 0x01, 0x02, 0x02, 0x03, 0x04, 0x05];
    assert_eq!(property.generate(), expected);
}

#[test]
fn generate_variable_byte() {
    let mut property = Property {
        values: BTreeMap::new(),
    };

    property.values.insert(
        SubscriptionIdentifier,
        DataType::VariableByteInteger(VariableByte::Four(268435455)),
    );

    let expected: Vec<u8> = vec![0x00, 0x01, 0x0b, 0xFF, 0xFF, 0xFF, 0x7F];
    assert_eq!(property.generate(), expected);
}

#[test]
fn generate_binary_data() {
    let mut property = Property {
        values: BTreeMap::new(),
    };

    let data: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
    property
        .values
        .insert(CorrelationData, DataType::BinaryData(data));

    let expected: Vec<u8> = vec![
        0x00, 0x01, 0x09, 0, 10, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
    ];
    assert_eq!(property.generate(), expected);
}

#[test]
fn generate_utf8_string() {
    let mut property = Property {
        values: BTreeMap::new(),
    };

    property.values.insert(
        ServerReference,
        DataType::Utf8EncodedString("hello world".to_string()),
    );

    let expected: Vec<u8> = vec![
        0x00, 0x01, 0x1c, 0, 11, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100,
    ];

    assert_eq!(property.generate(), expected);
}

#[test]
fn generate_utf8_string_pair() {
    let mut property = Property {
        values: BTreeMap::new(),
    };
    property.values.insert(
        UserProperty,
        DataType::Utf8StringPair("hello world".to_string(), "foo bar".to_string()),
    );

    let expected: Vec<u8> = vec![
        0x00, 0x01, 0x26, 0, 11, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 0, 7, 102,
        111, 111, 32, 98, 97, 114,
    ];
    assert_eq!(property.generate(), expected);
}

#[test]
fn generate_all() {
    let mut property = Property {
        values: BTreeMap::new(),
    };

    property
        .values
        .insert(PayloadFormatIndicator, DataType::Byte(255));

    property
        .values
        .insert(ServerKeepAlive, DataType::TwoByteInteger(515));

    property
        .values
        .insert(MessageExpiryInterval, DataType::FourByteInteger(33752069));

    property.values.insert(
        SubscriptionIdentifier,
        DataType::VariableByteInteger(VariableByte::Four(268435455)),
    );

    property.values.insert(
        CorrelationData,
        DataType::BinaryData(vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
        ]),
    );

    property.values.insert(
        ServerReference,
        DataType::Utf8EncodedString("hello world".to_string()),
    );

    property.values.insert(
        UserProperty,
        DataType::Utf8StringPair("hello world".to_string(), "foo bar".to_string()),
    );

    let expected = all_data();
    assert_eq!(property.generate(), expected);
}
