use std::convert::TryFrom;
use std::io;
use std::io::prelude::*;
use std::string::String;

/**
 * 1.5 Data representation
 * https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901006
 */
#[derive(Debug, PartialEq)]
pub enum Type {
    Byte(u8),
    TwoByteInteger(u16),
    FourByteInteger(u32),
    Utf8EncodedString(String),
    BinaryData(Vec<u8>),
    Utf8StringPair(String, String),
}

impl From<Type> for u16 {
    fn from(t: Type) -> Self {
        if let Type::TwoByteInteger(value) = t {
            return value;
        } else {
            return 0;
        }
    }
}

impl Type {
    /**
     * 1.5.1 Bits
     * https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901007
     * Bits in a byte are labelled 7 to 0. Bit number 7 is the most significant
     * bit, the least significant bit is assigned bit number 0.
     */
    pub fn parse_byte<R>(mut reader: R) -> Self
    where
        R: io::Read,
    {
        let mut buffer = [0; 1];
        reader.read(&mut buffer).expect("Reading error");
        return Self::Byte(u8::from_be_bytes(buffer));
    }

    /**
     * 1.5.2 Two Byte Integer
     * https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901008
     * Two Byte Integer data values are 16-bit unsigned integers in big-endian
     * order: the high order byte precedes the lower order byte. This means that a
     * 16-bit word is presented on the network as Most Significant Byte (MSB),
     * followed by Least Significant Byte (LSB).
     */
    pub fn parse_two_byte_int<R>(mut reader: R) -> Self
    where
        R: io::Read,
    {
        let mut buffer = [0; 2];
        reader.read(&mut buffer).expect("Reading error");
        return Self::TwoByteInteger(u16::from_be_bytes(buffer));
    }

    /**
     * 1.5.3 Four Byte Integer
     * https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901009
     * Four Byte Integer data values are 32-bit unsigned integers in big-endian
     * order: the high order byte precedes the successively lower order bytes.
     * This means that a 32-bit word is presented on the network as Most
     * Significant Byte (MSB), followed by the next most Significant Byte (MSB),
     * followed by the next most Significant Byte (MSB), followed by Least
     * Significant Byte (LSB).
     */
    pub fn parse_four_byte_int<R>(mut reader: R) -> Self
    where
        R: io::Read,
    {
        let mut buffer = [0; 4];
        reader.read(&mut buffer).expect("Reading error");
        return Self::FourByteInteger(u32::from_be_bytes(buffer));
    }

    /**
     * 1.5.4 UTF-8 Encoded String
     * https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901010
     * Text fields within the MQTT Control Packets described later are encoded as
     * UTF-8 strings. UTF-8 [RFC3629] is an efficient encoding of Unicode
     * characters that optimizes the encoding of ASCII characters in support of
     * text-based communications.
     *
     * Each of these strings is prefixed with a Two Byte Integer length field that
     * gives the number of bytes in a UTF-8 encoded string itself, as illustrated
     * in Figure 1.1 Structure of UTF-8 Encoded Strings below. Consequently, the
     * maximum size of a UTF-8 Encoded String is 65,535 bytes.
     *
     * Unless stated otherwise all UTF-8 encoded strings can have any length in
     * the range 0 to 65,535 bytes.
     */
    fn parse_string<R>(mut reader: R) -> String
    where
        R: io::Read,
    {
        // get the expected length of the string
        let mut length_buffer = [0; 2];
        reader.read(&mut length_buffer).expect("Reading error");
        let length = u16::from_be_bytes(length_buffer);

        // read the string
        let mut handle = reader.take(u64::from(length));
        let mut buffer = vec![];
        handle.read_to_end(&mut buffer).expect("Reading error");
        return String::from_utf8(buffer).unwrap();
    }

    pub fn parse_utf8_string<R>(reader: R) -> Self
    where
        R: io::Read,
    {
        return Self::Utf8EncodedString(Self::parse_string(reader));
    }

    /**
     * 1.5.5 Variable Byte Integer
     * https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901011
     * The Variable Byte Integer is encoded using an encoding scheme which uses a
     * single byte for values up to 127. Larger values are handled as follows. The
     * least significant seven bits of each byte encode the data, and the most
     * significant bit is used to indicate whether there are bytes following in
     * the representation. Thus, each byte encodes 128 values and a "continuation
     * bit". The maximum number of bytes in the Variable Byte Integer field is four.
     * The encoded value MUST use the minimum number of bytes necessary to represent
     * the value [MQTT-1.5.5-1]. This is shown in Table 1‑1 Size of Variable Byte
     * Integer.
     */
    pub fn parse_variable_byte_int<R>(mut reader: R) -> Self
    where
        R: io::Read,
    {
        let mut more: bool = true;
        let mut multiplier: u32 = 1;
        let mut value: u32 = 0;

        while more {
            let mut b = [0; 1];
            reader.read(&mut b).expect("Reading error");
            value = value + u32::from(b[0] & 127) * multiplier;

            if multiplier > (128 * 128 * 128) {
                panic!("Malformed VariableByteInteger");
            }

            multiplier = multiplier * 128;

            if (b[0] & 128) == 0 {
                more = false;
            }
        }

        if value <= u8::max_value().into() {
            return Self::Byte(u8::try_from(value).unwrap());
        } else if value <= u16::max_value().into() {
            return Self::TwoByteInteger(u16::try_from(value).unwrap());
        } else {
            return Self::FourByteInteger(value);
        }
    }

    /**
     * 1.5.6 Binary Data
     * https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901012
     * Binary Data is represented by a Two Byte Integer length which indicates the
     * number of data bytes, followed by that number of bytes. Thus, the length of
     * Binary Data is limited to the range of 0 to 65,535 Bytes.
     */
    pub fn parse_binary_data<R>(mut reader: R) -> Self
    where
        R: io::Read,
    {
        // get the expected length of the string
        let mut length_buffer = [0; 2];
        reader.read(&mut length_buffer).expect("Reading error");
        let length = u16::from_be_bytes(length_buffer);

        // read the data
        let mut handle = reader.take(u64::from(length));
        let mut buffer = vec![];
        handle.read_to_end(&mut buffer).expect("Reading error");

        return Self::BinaryData(buffer);
    }

    /**
     * 1.5.7 UTF-8 String Pair
     * https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901013
     * A UTF-8 String Pair consists of two UTF-8 Encoded Strings. This data type
     * is used to hold name-value pairs. The first string serves as the name, and
     * the second string contains the value.
     *
     * Both strings MUST comply with the requirements for UTF-8 Encoded Strings
     * [MQTT-1.5.7-1]. If a receiver (Client or Server) receives a string pair
     * which does not meet these requirements it is a Malformed Packet. Refer to
     * section 4.13 for information about handling errors.
     */
    pub fn parse_utf8_string_pair<R>(mut reader: R) -> Self
    where
        R: io::Read,
    {
        let str_one = Self::parse_string(&mut reader);
        let str_two = Self::parse_string(&mut reader);

        return Self::Utf8StringPair(str_one, str_two);
    }

    /**
     * Used by into_bytes() for calculating length for strings, string pairs, and binary data.
     * The length of arrays is limited to the range of 0 to 65,535 bytes. Because of that we
     * need to convert usize to a two byte u8 array.
     */
    fn calculate_length(data: Vec<u8>) -> Vec<u8> {
        if data.len() > 65535 {
            panic!("The max length of data is 65,535 bytes.");
        }
        let mut length = data.len().to_le_bytes()[0..2].to_vec();
        length.reverse();
        return [&length[..], &data[..]].concat();
    }

    /**
     * Convert Type variants into u8 vectors.
     */
    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            Self::Byte(value) => value.to_be_bytes().to_vec(),
            Self::TwoByteInteger(value) => value.to_be_bytes().to_vec(),
            Self::FourByteInteger(value) => value.to_be_bytes().to_vec(),
            Self::Utf8EncodedString(value) => Self::calculate_length(value.into_bytes()),
            Self::BinaryData(value) => Self::calculate_length(value.to_vec()),
            Self::Utf8StringPair(one, two) => [
                Self::calculate_length(one.into_bytes()),
                Self::calculate_length(two.into_bytes()),
            ]
            .concat(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Type;
    use std::io;

    #[test]
    fn byte() {
        let reader: Vec<u8> = vec![0xFF, 0x02];
        let byte = Type::parse_byte(&*reader);
        assert_eq!(byte, super::Type::Byte(255));
    }

    #[test]
    fn two_byte() {
        let reader: Vec<u8> = vec![0x01, 0x02, 0x03];
        let two = Type::parse_two_byte_int(&*reader);
        assert_eq!(two, super::Type::TwoByteInteger(258));
    }

    #[test]
    fn type_into() {
        let mut reader: Vec<u8> = vec![0x01, 0x02, 0x03];
        let two = Type::parse_two_byte_int(&*reader);
        let mut check: u16 = two.into();
        assert_eq!(258, check);

        // any other type should return 0 for now
        reader = vec![0x01, 0x02, 0x03, 0x04, 0x05];
        let four = Type::parse_four_byte_int(&*reader);
        check = four.into();
        assert_eq!(0, check);
    }

    #[test]
    fn four_byte() {
        let reader: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04, 0x05];
        let four = Type::parse_four_byte_int(&*reader);
        assert_eq!(four, Type::FourByteInteger(16909060));
    }

    #[test]
    fn utf8_string() {
        let data: Vec<u8> = vec![
            0, 11, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 100, 100, 100,
        ];

        let reader = io::BufReader::new(&*data);
        let result = Type::parse_utf8_string(reader);
        assert_eq!(result, Type::Utf8EncodedString(String::from("hello world")));
    }

    #[test]
    fn variable_byte_one() {
        let mut vari: Vec<u8> = vec![0x00];
        let mut vari_type = Type::parse_variable_byte_int(&*vari);
        assert_eq!(vari_type, Type::Byte(0));

        vari = vec![0x7F];
        vari_type = Type::parse_variable_byte_int(&*vari);
        assert_eq!(vari_type, Type::Byte(127));
    }

    #[test]
    fn variable_byte_two() {
        let mut vari: Vec<u8> = vec![0x80, 0x01];
        let mut vari_type = Type::parse_variable_byte_int(&*vari);
        assert_eq!(vari_type, Type::Byte(128));

        vari = vec![0xFF, 0x7F];
        vari_type = Type::parse_variable_byte_int(&*vari);
        assert_eq!(vari_type, Type::TwoByteInteger(16383));
    }

    #[test]
    fn variable_byte_three() {
        let mut vari: Vec<u8> = vec![0x80, 0x80, 0x01];
        let mut vari_type = Type::parse_variable_byte_int(&*vari);
        assert_eq!(vari_type, Type::TwoByteInteger(16384));

        vari = vec![0xFF, 0xFF, 0x7F];
        vari_type = Type::parse_variable_byte_int(&*vari);
        assert_eq!(vari_type, Type::FourByteInteger(2097151));
    }

    #[test]
    fn variable_byte_four() {
        let mut vari: Vec<u8> = vec![0x80, 0x80, 0x80, 0x01];
        let mut vari_type = Type::parse_variable_byte_int(&*vari);
        assert_eq!(vari_type, Type::FourByteInteger(2097152));

        vari = vec![0xFF, 0xFF, 0xFF, 0x7F];
        vari_type = Type::parse_variable_byte_int(&*vari);
        assert_eq!(vari_type, Type::FourByteInteger(268435455));
    }

    #[test]
    #[should_panic]
    fn variable_byte_panic() {
        let vari: Vec<u8> = vec![0xFF, 0xFF, 0xFF, 0xFF];
        let _vari_type = Type::parse_variable_byte_int(&*vari);
    }

    #[test]
    fn binary_data() {
        let data: Vec<u8> = vec![
            0, 10, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A,
        ];

        let reader = io::BufReader::new(&*data);
        let result = Type::parse_binary_data(reader);

        let expected: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
        assert_eq!(result, Type::BinaryData(expected));
    }

    #[test]
    fn string_pair() {
        let data: Vec<u8> = vec![
            0, 11, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 0, 7, 102, 111, 111, 32,
            98, 97, 114, 1, 1, 1, 1,
        ];

        let reader = io::BufReader::new(&*data);
        let result = Type::parse_utf8_string_pair(reader);

        assert_eq!(
            result,
            Type::Utf8StringPair(String::from("hello world"), String::from("foo bar"))
        );
    }

    #[test]
    fn byte_into_bytes() {
        let value = Type::Byte(255);
        let expected: Vec<u8> = vec![0xFF];
        assert_eq!(value.into_bytes(), expected);
    }

    #[test]
    fn two_byte_int_into_bytes() {
        let value = Type::TwoByteInteger(258);
        let expected: Vec<u8> = vec![0x01, 0x02];
        assert_eq!(value.into_bytes(), expected);
    }

    #[test]
    fn four_byte_into_bytes() {
        let value = Type::FourByteInteger(16909060);
        let expected: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04];
        assert_eq!(value.into_bytes(), expected);
    }

    #[test]
    fn utf8_string_into_bytes() {
        let value = Type::Utf8EncodedString("hello world".to_string());
        let expected: Vec<u8> = vec![0, 11, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100];
        assert_eq!(value.into_bytes(), expected);
    }

    #[test]
    fn utf8_string_pair_into_bytes() {
        let value = Type::Utf8StringPair("hello world".to_string(), "foo bar".to_string());
        let expected: Vec<u8> = vec![
            0, 11, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 0, 7, 102, 111, 111, 32,
            98, 97, 114,
        ];
        assert_eq!(value.into_bytes(), expected);
    }

    #[test]
    fn binary_data_into_bytes() {
        let data: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
        let value = Type::BinaryData(data);

        let expected: Vec<u8> = vec![
            0, 10, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
        ];
        assert_eq!(value.into_bytes(), expected);
    }

    #[test]
    #[should_panic]
    fn into_bytes_max_length() {
        let data = [0u8; 65536];
        let value = Type::BinaryData(data.to_vec());
        let _should_panic = value.into_bytes();
    }
}
