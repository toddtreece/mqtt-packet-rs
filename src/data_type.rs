use std::io;
use std::io::prelude::*;

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
pub type VariableByteInteger = Type<u64>;

impl TypeParser for VariableByteInteger {
    fn new<R>(mut reader: R) -> Type<u64>
    where
        R: io::Read,
    {
        let mut more: bool = true;
        let mut multiplier: u64 = 1;
        let mut value: u64 = 0;

        while more {
            let mut b = [0; 1];
            reader.read(&mut b).expect("Reading error");
            value = value + u64::from(b[0] & 127) * multiplier;

            if multiplier > (128 * 128 * 128) {
                panic!("Malformed VariableByteInteger");
            }

            multiplier = multiplier * 128;

            if (b[0] & 128) == 0 {
                more = false;
            }
        }

        Type { value: value }
    }
}

/**
 * 1.5.6 Binary Data
 * https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901012
 * Binary Data is represented by a Two Byte Integer length which indicates the
 * number of data bytes, followed by that number of bytes. Thus, the length of
 * Binary Data is limited to the range of 0 to 65,535 Bytes.
 */
pub type BinaryData = Type<Vec<u8>>;

impl TypeParser for BinaryData {
    fn new<R>(mut reader: R) -> Type<Vec<u8>>
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

        Type { value: buffer }
    }
}

// 1.5.7 UTF-8 String Pair
// https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901013
//pub struct Utf8StringPair(String, String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte() {
        let reader: Vec<u8> = vec![0xFF, 0x02];
        let byte = Byte::new(&*reader);
        assert_eq!(byte.value, 255);
    }

    #[test]
    fn two_byte() {
        let reader: Vec<u8> = vec![0x01, 0x02, 0x03];
        let two = TwoByteInteger::new(&*reader);
        assert_eq!(two.value, 258);
    }

    #[test]
    fn four_byte() {
        let reader: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04, 0x05];
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

    #[test]
    fn variable_byte_one() {
        let mut vari: Vec<u8> = vec![0x00];
        let mut vari_type = VariableByteInteger::new(&*vari);
        assert_eq!(vari_type.value, 0);

        vari = vec![0x7F];
        vari_type = VariableByteInteger::new(&*vari);
        assert_eq!(vari_type.value, 127);
    }

    #[test]
    fn variable_byte_two() {
        let mut vari: Vec<u8> = vec![0x80, 0x01];
        let mut vari_type = VariableByteInteger::new(&*vari);
        assert_eq!(vari_type.value, 128);

        vari = vec![0xFF, 0x7F];
        vari_type = VariableByteInteger::new(&*vari);
        assert_eq!(vari_type.value, 16383);
    }

    #[test]
    fn variable_byte_three() {
        let mut vari: Vec<u8> = vec![0x80, 0x80, 0x01];
        let mut vari_type = VariableByteInteger::new(&*vari);
        assert_eq!(vari_type.value, 16384);

        vari = vec![0xFF, 0xFF, 0x7F];
        vari_type = VariableByteInteger::new(&*vari);
        assert_eq!(vari_type.value, 2097151);
    }

    #[test]
    fn variable_byte_four() {
        let mut vari: Vec<u8> = vec![0x80, 0x80, 0x80, 0x01];
        let mut vari_type = VariableByteInteger::new(&*vari);
        assert_eq!(vari_type.value, 2097152);

        vari = vec![0xFF, 0xFF, 0xFF, 0x7F];
        vari_type = VariableByteInteger::new(&*vari);
        assert_eq!(vari_type.value, 268435455);
    }

    #[test]
    fn binary_data() {
        let data: Vec<u8> = vec![
            0, 10, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A,
        ];

        let reader = io::BufReader::new(&*data);
        let result = BinaryData::new(reader);

        let expected: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
        assert_eq!(result.value, expected);
    }
}
