use mqtt_packet::data_type::Type;
use mqtt_packet::property::Property;

#[test]
fn byte() {
    let reader: Vec<u8> = vec![0x00, 0x02, 0x01, 0xFF, 0x24, 0x02];
    let property = Property::parse(&*reader);

    match property.values.get("payloadFormatIndicator") {
        Some(value) => assert_eq!(value, &Type::Byte(255)),
        None => panic!("Not a valid property"),
    }

    match property.values.get("maximumQos") {
        Some(value) => assert_eq!(value, &Type::Byte(2)),
        None => panic!("Not a valid property"),
    }
}
