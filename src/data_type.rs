use std::io;

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

// 1.5.4 UTF-8 Encoded String
// https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901010
//pub type Utf8EncodedString = String;

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
}
