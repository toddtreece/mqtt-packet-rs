use std::io;
use std::io::prelude::*;
use std::str;

/**
 * 1.5 Data representation
 * https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901006
 */
pub struct Type<T> {
    value: T,
}

pub trait TypeParser {
    fn new<R>(reader: R) -> Self
    where
        R: io::Read;
}

/**
 * 1.5.1 Bits
 * https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901007
 * Bits in a byte are labelled 7 to 0. Bit number 7 is the most significant
 * bit, the least significant bit is assigned bit number 0.
 */
pub type Byte = Type<u8>;

impl TypeParser for Byte {
    fn new<R>(mut reader: R) -> Type<u8>
    where
        R: io::Read,
    {
        let mut buffer = [0; 1];
        reader.read(&mut buffer).expect("Reading error");
        println!("{:x?}", buffer);
        Type {
            value: u8::from_be_bytes(buffer),
        }
    }
}

/**
 * 1.5.2 Two Byte Integer
 * https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901008
 * Two Byte Integer data values are 16-bit unsigned integers in big-endian
 * order: the high order byte precedes the lower order byte. This means that a
 * 16-bit word is presented on the network as Most Significant Byte (MSB),
 * followed by Least Significant Byte (LSB).
 */
pub type TwoByteInteger = Type<u16>;

impl TypeParser for TwoByteInteger {
    fn new<R>(mut reader: R) -> Type<u16>
    where
        R: io::Read,
    {
        let mut buffer = [0; 2];
        reader.read(&mut buffer).expect("Reading error");
        Type {
            value: u16::from_be_bytes(buffer),
        }
    }
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
pub type FourByteInteger = Type<u32>;

impl TypeParser for FourByteInteger {
    fn new<R>(mut reader: R) -> Type<u32>
    where
        R: io::Read,
    {
        let mut buffer = [0; 4];
        reader.read(&mut buffer).expect("Reading error");
        Type {
            value: u32::from_be_bytes(buffer),
        }
    }
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
pub type Utf8EncodedString = Type<String>;

impl TypeParser for Utf8EncodedString {
    fn new<R>(mut reader: R) -> Type<String>
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

        Type {
            value: String::from_utf8(buffer).unwrap(),
        }
    }
}

// 1.5.5 Variable Byte Integer
// https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901011
//pub type VariableByteInteger = i64;

// 1.5.6 Binary Data
// https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901012
//pub type BinaryData = Vec<u8>;

// 1.5.7 UTF-8 String Pair
// https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901013
//pub struct Utf8StringPair(String, String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte() {
        let reader: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x0a, 0x0b];

        let byte = Byte::new(&*reader);
        assert_eq!(byte.value, 1);
    }

    #[test]
    fn two_byte() {
        let reader: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x0a, 0x0b];

        let two = TwoByteInteger::new(&*reader);
        assert_eq!(two.value, 258);
    }

    #[test]
    fn four_byte() {
        let reader: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x0a, 0x0b];

        let four = FourByteInteger::new(&*reader);
        assert_eq!(four.value, 16909060);
    }

    #[test]
    fn utf8_string() {
        let data: Vec<u8> = vec![
            0, 11, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 100, 100, 100,
        ];
        let reader = io::BufReader::new(&*data);

        let result = Utf8EncodedString::new(reader);
        assert_eq!(result.value, "hello world");
    }
}
