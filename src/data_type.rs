use crate::Error;
use std::convert::{TryFrom, TryInto};
use std::io;
use std::io::prelude::*;
use std::string::String;

#[derive(Debug, PartialEq)]
pub enum VariableByte {
  One(u8),
  Two(u16),
  Three(u32),
  Four(u32),
}

/// Data types defined by the MQTT v5 spec.
#[derive(Debug, PartialEq)]
pub enum DataType {
  Byte(u8),
  TwoByteInteger(u16),
  FourByteInteger(u32),
  VariableByteInteger(VariableByte),
  Utf8EncodedString(String),
  BinaryData(Vec<u8>),
  Utf8StringPair(String, String),
}

impl From<DataType> for u16 {
  fn from(t: DataType) -> Self {
    if let DataType::TwoByteInteger(value) = t {
      return value;
    } else {
      return 0;
    }
  }
}

impl DataType {
  /// Reads one byte from the reader and attempts to convert the byte to DataType::Byte (u8).
  ///
  /// [1.5.1 Bits](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901007)
  ///
  /// Bits in a byte are labelled 7 to 0. Bit number 7 is the most significant
  /// bit, the least significant bit is assigned bit number 0.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use mqtt_packet::DataType;
  /// use std::io;
  ///
  /// let data: Vec<u8> = vec![0xFF, 0x02];
  /// let mut reader = io::BufReader::new(&data[..]);
  /// let byte = DataType::parse_byte(&mut reader).unwrap();
  /// assert_eq!(byte, DataType::Byte(255));
  /// ```
  pub fn parse_byte<R: io::Read>(reader: &mut R) -> Result<Self, Error> {
    let mut buffer = [0; 1];
    reader.read(&mut buffer)?;
    return Ok(Self::Byte(u8::from_be_bytes(buffer)));
  }

  /// Reads two bytes from the reader and attempts to convert the bytes to DataType::TwoByteInteger (u16).
  ///
  /// [1.5.2 Two Byte Integer](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901008)
  ///
  /// Two Byte Integer data values are 16-bit unsigned integers in big-endian
  /// order: the high order byte precedes the lower order byte. This means that a
  /// 16-bit word is presented on the network as Most Significant Byte (MSB),
  /// followed by Least Significant Byte (LSB).
  ///
  /// # Examples
  ///
  /// ```rust
  /// use mqtt_packet::DataType;
  /// use std::io;
  ///
  /// let data: Vec<u8> = vec![0x01, 0x02, 0x03];
  /// let mut reader = io::BufReader::new(&data[..]);
  /// let two = DataType::parse_two_byte_int(&mut reader).unwrap();
  /// assert_eq!(two, DataType::TwoByteInteger(258));
  /// ```
  pub fn parse_two_byte_int<R: io::Read>(reader: &mut R) -> Result<Self, Error> {
    let mut buffer = [0; 2];
    reader.read(&mut buffer)?;
    return Ok(Self::TwoByteInteger(u16::from_be_bytes(buffer)));
  }

  /// Reads four bytes from the reader and attempts to convert the bytes to DataType::FourByteInteger (u32).
  ///
  /// [1.5.3 Four Byte Integer](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901009)
  ///
  /// Four Byte Integer data values are 32-bit unsigned integers in big-endian
  /// order: the high order byte precedes the successively lower order bytes.
  /// This means that a 32-bit word is presented on the network as Most
  /// Significant Byte (MSB), followed by the next most Significant Byte (MSB),
  /// followed by the next most Significant Byte (MSB), followed by Least
  /// Significant Byte (LSB).
  ///
  /// # Examples
  ///
  /// ```rust
  /// use mqtt_packet::DataType;
  /// use std::io;
  ///
  /// let data: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04, 0x05];
  /// let mut reader = io::BufReader::new(&data[..]);
  /// let four = DataType::parse_four_byte_int(&mut reader).unwrap();
  /// assert_eq!(four, DataType::FourByteInteger(16909060));
  /// ```
  pub fn parse_four_byte_int<R: io::Read>(reader: &mut R) -> Result<Self, Error> {
    let mut buffer = [0; 4];
    reader.read(&mut buffer)?;
    return Ok(Self::FourByteInteger(u32::from_be_bytes(buffer)));
  }

  fn parse_string<R: io::Read>(reader: &mut R) -> Result<String, Error> {
    // get the expected length of the string
    let mut length_buffer = [0; 2];

    reader.read(&mut length_buffer)?;

    let length = u16::from_be_bytes(length_buffer);

    // read the string
    let mut handle = reader.take(u64::from(length));
    let mut buffer = vec![];
    handle.read_to_end(&mut buffer)?;
    let s = String::from_utf8(buffer)?;

    return Ok(s);
  }

  /// Reads bytes from the reader and attempts to convert the bytes to DataType::Utf8EncodedString (String).
  ///
  /// [1.5.4 UTF-8 Encoded String](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901010)
  ///
  /// Text fields within the MQTT Control Packets described later are encoded as
  /// UTF-8 strings. UTF-8 is an efficient encoding of Unicode
  /// characters that optimizes the encoding of ASCII characters in support of
  /// text-based communications.
  ///
  /// Each of these strings is prefixed with a Two Byte Integer length field that
  /// gives the number of bytes in a UTF-8 encoded string itself, as illustrated
  /// in Figure 1.1 Structure of UTF-8 Encoded Strings below. Consequently, the
  /// maximum size of a UTF-8 Encoded String is 65,535 bytes.
  ///
  /// Unless stated otherwise all UTF-8 encoded strings can have any length in
  /// the range 0 to 65,535 bytes.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use std::io;
  /// use mqtt_packet::DataType;
  ///
  /// let data: Vec<u8> = vec![
  ///   0, 11, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 100, 100, 100,
  /// ];
  ///
  /// let mut reader = io::BufReader::new(&data[..]);
  /// let result = DataType::parse_utf8_string(&mut reader).unwrap();
  /// assert_eq!(
  ///   result,
  ///   DataType::Utf8EncodedString(String::from("hello world"))
  /// );
  /// ```
  pub fn parse_utf8_string<R: io::Read>(reader: &mut R) -> Result<Self, Error> {
    let s = Self::parse_string(reader)?;
    return Ok(Self::Utf8EncodedString(s));
  }

  /// Reads bytes from the reader and attempts to convert the bytes to DataType::VariableByteInteger (u8, 16, or u32).
  ///
  /// [1.5.5 Variable Byte Integer](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901011)
  ///
  /// The Variable Byte Integer is encoded using an encoding scheme which uses a
  /// single byte for values up to 127. Larger values are handled as follows. The
  /// least significant seven bits of each byte encode the data, and the most
  /// significant bit is used to indicate whether there are bytes following in
  /// the representation. Thus, each byte encodes 128 values and a "continuation
  /// bit". The maximum number of bytes in the Variable Byte Integer field is four.
  /// The encoded value MUST use the minimum number of bytes necessary to represent
  /// the value.
  ///
  /// The algorithm for decoding a Variable Byte Integer type is as follows:
  /// ```txt
  /// multiplier = 1
  /// value = 0
  ///
  /// do
  ///    encodedByte = 'next byte from stream'
  ///    value += (encodedByte AND 127) * multiplier
  ///
  ///    if (multiplier > 128*128*128)
  ///       throw Error(Malformed Variable Byte Integer)
  ///    multiplier *= 128
  /// while ((encodedByte AND 128) != 0)
  /// ```
  ///
  /// # Examples
  ///
  /// ```rust
  /// use mqtt_packet::{DataType, VariableByte};
  /// use std::io;
  ///
  /// let data: Vec<u8> = vec![0x80, 0x80, 0x80, 0x01];
  /// let mut reader = io::BufReader::new(&data[..]);
  /// let mut vari_type = DataType::parse_variable_byte_int(&mut reader).unwrap();
  ///
  /// assert_eq!(
  ///   vari_type,
  ///   DataType::VariableByteInteger(VariableByte::Four(2097152))
  /// );
  /// ```
  pub fn parse_variable_byte_int<R: io::Read>(reader: &mut R) -> Result<Self, Error> {
    let mut multiplier: i32 = 1;
    let mut value: i32 = 0;

    loop {
      let mut b = [0; 1];
      reader.read(&mut b)?;

      value = value + i32::from(b[0] & 127) * multiplier;

      if multiplier > (128 * 128 * 128) {
        return Err(Error::ParseError);
      }

      multiplier = multiplier * 128;

      if (b[0] & 128) == 0 {
        break;
      }
    }

    let parsed = match value {
      n if n <= 127 => Self::VariableByteInteger(VariableByte::One(u8::try_from(value)?)),
      n if n <= 16383 => Self::VariableByteInteger(VariableByte::Two(u16::try_from(value)?)),
      n if n <= 2097151 => Self::VariableByteInteger(VariableByte::Three(u32::try_from(value)?)),
      _ => Self::VariableByteInteger(VariableByte::Four(u32::try_from(value)?)),
    };

    return Ok(parsed);
  }

  /// Reads bytes from the reader and attempts to convert the bytes to DataType::BinaryData (Vec<u8>).
  ///
  /// [1.5.6 Binary Data](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901012)
  ///
  /// Binary Data is represented by a Two Byte Integer length which indicates the
  /// number of data bytes, followed by that number of bytes. Thus, the length of
  /// Binary Data is limited to the range of 0 to 65,535 Bytes.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use mqtt_packet::DataType;
  /// use std::io;
  ///
  /// let data: Vec<u8> = vec![
  ///   0, 10, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A,
  /// ];
  ///
  /// let mut reader = io::BufReader::new(&data[..]);
  /// let result = DataType::parse_binary_data(&mut reader).unwrap();
  ///
  /// let expected: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
  /// assert_eq!(result, DataType::BinaryData(expected));
  /// ```
  pub fn parse_binary_data<R: io::Read>(reader: &mut R) -> Result<Self, Error> {
    // determine the length of the binary data
    let mut length_buffer = [0; 2];
    reader.read(&mut length_buffer)?;
    let length = u16::from_be_bytes(length_buffer);

    // read the data
    let mut handle = reader.take(u64::from(length));
    let mut buffer = vec![];
    handle.read_to_end(&mut buffer)?;

    return Ok(Self::BinaryData(buffer));
  }

  /// Reads bytes from the reader and attempts to convert the bytes to DataType::Utf8StringPair (String, String).
  ///
  ///  [1.5.7 UTF-8 String Pair](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901013)
  ///
  /// A UTF-8 String Pair consists of two UTF-8 Encoded Strings. This data type
  /// is used to hold name-value pairs. The first string serves as the name, and
  /// the second string contains the value.
  ///
  /// Both strings MUST comply with the requirements for UTF-8 Encoded Strings.
  /// If a receiver (Client or Server) receives a string pair
  /// which does not meet these requirements it is a Malformed Packet. Refer to
  /// section 4.13 for information about handling errors.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use mqtt_packet::DataType;
  /// use std::io;
  /// ```
  pub fn parse_utf8_string_pair<R: io::Read>(reader: &mut R) -> Result<Self, Error> {
    let str_one = Self::parse_string(reader)?;
    let str_two = Self::parse_string(reader)?;

    return Ok(Self::Utf8StringPair(str_one, str_two));
  }

  /// Used by into_bytes() for calculating length for strings, string pairs, and binary data.
  /// The length of arrays is limited to the range of 0 to 65,535 bytes. Because of that we
  /// need to convert usize to a two byte u8 array.
  fn calculate_length(data: Vec<u8>) -> Result<Vec<u8>, Error> {
    if data.len() > 65535 {
      return Err(Error::GenerateError);
    }

    let length = u16::try_from(data.len() & 0xFFFF)
      .unwrap()
      .to_be_bytes()
      .to_vec();

    return Ok([&length[..], &data[..]].concat());
  }

  /// Used by into_bytes() to format variable byte ints into the format defined in the
  /// MQTT v5 spec.
  ///
  /// The algorithm for encoding a non-negative integer (X) into the Variable Byte
  /// Integer encoding scheme is as follows:
  ///
  /// ```txt
  /// do
  ///    encodedByte = X MOD 128
  ///    X = X DIV 128
  ///    // if there are more data to encode, set the top bit of this byte
  ///    if (X > 0)
  ///       encodedByte = encodedByte OR 128
  ///    endif
  ///    'output' encodedByte
  /// while (X > 0)
  /// ```
  ///
  /// Where MOD is the modulo operator (% in C), DIV is integer division (/ in C),
  /// and OR is bit-wise or (| in C).
  fn encode_variable_byte(data: &VariableByte) -> Result<Vec<u8>, Error> {
    let mut bytes = vec![];
    let mut number: u32 = match data {
      VariableByte::One(value) => u32::from(*value),
      VariableByte::Two(value) => u32::from(*value),
      VariableByte::Three(value) => u32::from(*value),
      VariableByte::Four(value) => u32::from(*value),
    };

    if number > 268435455 {
      return Err(Error::GenerateError);
    }

    loop {
      // we are safe to unwrap here because (number % 128) will neveer be bigger than 127
      let mut encoded_byte: u8 = (number % 128).try_into().unwrap();
      number = number / 128;

      if number > 0 {
        encoded_byte = encoded_byte | 128;
        bytes.push(encoded_byte);
      } else {
        bytes.push(encoded_byte);
        break;
      }
    }

    return Ok(bytes);
  }

  /// Convert DataType variants into u8 vectors.
  pub fn into_bytes(&self) -> Result<Vec<u8>, Error> {
    let bytes = match self {
      Self::Byte(value) => value.to_be_bytes().to_vec(),
      Self::TwoByteInteger(value) => value.to_be_bytes().to_vec(),
      Self::FourByteInteger(value) => value.to_be_bytes().to_vec(),
      Self::VariableByteInteger(value) => Self::encode_variable_byte(value)?,
      Self::Utf8EncodedString(value) => Self::calculate_length(value.as_bytes().to_vec())?,
      Self::BinaryData(value) => Self::calculate_length(value.to_vec())?,
      Self::Utf8StringPair(one, two) => [
        Self::calculate_length(one.as_bytes().to_vec())?,
        Self::calculate_length(two.as_bytes().to_vec())?,
      ]
      .concat(),
    };

    return Ok(bytes);
  }
}

#[cfg(test)]
mod tests {
  use super::{DataType, VariableByte};
  use crate::Error;
  use std::io;

  #[test]
  fn type_into() {
    let data: Vec<u8> = vec![0x01, 0x02, 0x03];
    let mut reader = io::BufReader::new(&data[..]);
    let two = DataType::parse_two_byte_int(&mut reader).unwrap();
    let mut check: u16 = two.into();
    assert_eq!(258, check);

    // any other type should return 0 for now
    let zero = vec![0x01, 0x02, 0x03, 0x04, 0x05];
    reader = io::BufReader::new(&zero[..]);
    let four = DataType::parse_four_byte_int(&mut reader).unwrap();
    check = four.into();
    assert_eq!(0, check);
  }

  #[test]
  fn variable_byte_one() {
    let min: Vec<u8> = vec![0x00];
    let mut reader = io::BufReader::new(&min[..]);
    let mut vari_type = DataType::parse_variable_byte_int(&mut reader).unwrap();
    assert_eq!(
      vari_type,
      DataType::VariableByteInteger(VariableByte::One(0))
    );

    let max = vec![0x7F];
    reader = io::BufReader::new(&max[..]);
    vari_type = DataType::parse_variable_byte_int(&mut reader).unwrap();
    assert_eq!(
      vari_type,
      DataType::VariableByteInteger(VariableByte::One(127))
    );
  }

  #[test]
  fn variable_byte_two() {
    let min: Vec<u8> = vec![0x80, 0x01];
    let mut reader = io::BufReader::new(&min[..]);
    let mut vari_type = DataType::parse_variable_byte_int(&mut reader).unwrap();
    assert_eq!(
      vari_type,
      DataType::VariableByteInteger(VariableByte::Two(128))
    );

    let max: Vec<u8> = vec![0xFF, 0x7F];
    reader = io::BufReader::new(&max[..]);
    vari_type = DataType::parse_variable_byte_int(&mut reader).unwrap();
    assert_eq!(
      vari_type,
      DataType::VariableByteInteger(VariableByte::Two(16383))
    );
  }

  #[test]
  fn variable_byte_three() {
    let min: Vec<u8> = vec![0x80, 0x80, 0x01];
    let mut reader = io::BufReader::new(&min[..]);
    let mut vari_type = DataType::parse_variable_byte_int(&mut reader).unwrap();
    assert_eq!(
      vari_type,
      DataType::VariableByteInteger(VariableByte::Three(16384))
    );

    let max: Vec<u8> = vec![0xFF, 0xFF, 0x7F];
    reader = io::BufReader::new(&max[..]);
    vari_type = DataType::parse_variable_byte_int(&mut reader).unwrap();
    assert_eq!(
      vari_type,
      DataType::VariableByteInteger(VariableByte::Three(2097151))
    );
  }

  #[test]
  fn variable_byte_four() {
    let min: Vec<u8> = vec![0x80, 0x80, 0x80, 0x01];
    let mut reader = io::BufReader::new(&min[..]);
    let mut vari_type = DataType::parse_variable_byte_int(&mut reader).unwrap();
    assert_eq!(
      vari_type,
      DataType::VariableByteInteger(VariableByte::Four(2097152))
    );

    let max: Vec<u8> = vec![0xFF, 0xFF, 0xFF, 0x7F];
    reader = io::BufReader::new(&max[..]);
    vari_type = DataType::parse_variable_byte_int(&mut reader).unwrap();
    assert_eq!(
      vari_type,
      DataType::VariableByteInteger(VariableByte::Four(268435455))
    );
  }

  #[test]
  fn variable_byte_error() {
    let vari: Vec<u8> = vec![0xFF, 0xFF, 0xFF, 0xFF];
    let mut reader = io::BufReader::new(&vari[..]);
    let vari_err = DataType::parse_variable_byte_int(&mut reader).unwrap_err();
    assert_eq!(vari_err, Error::ParseError);
  }

  #[test]
  fn string_pair() {
    let data: Vec<u8> = vec![
      0, 11, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 0, 7, 102, 111, 111, 32, 98, 97,
      114, 1, 1, 1, 1,
    ];

    let mut reader = io::BufReader::new(&data[..]);
    let result = DataType::parse_utf8_string_pair(&mut reader).unwrap();

    assert_eq!(
      result,
      DataType::Utf8StringPair(String::from("hello world"), String::from("foo bar"))
    );
  }

  #[test]
  fn byte_into_bytes() {
    let value = DataType::Byte(255);
    let expected: Vec<u8> = vec![0xFF];
    assert_eq!(value.into_bytes().unwrap(), expected);
  }

  #[test]
  fn two_byte_int_into_bytes() {
    let value = DataType::TwoByteInteger(258);
    let expected: Vec<u8> = vec![0x01, 0x02];
    assert_eq!(value.into_bytes().unwrap(), expected);
  }

  #[test]
  fn four_byte_into_bytes() {
    let value = DataType::FourByteInteger(16909060);
    let expected: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04];
    assert_eq!(value.into_bytes().unwrap(), expected);
  }

  #[test]
  fn variable_byte_one_into_bytes() {
    let mut vari = DataType::VariableByteInteger(VariableByte::One(0));
    let mut expected: Vec<u8> = vec![0x00];
    assert_eq!(vari.into_bytes().unwrap(), expected);

    vari = DataType::VariableByteInteger(VariableByte::One(127));
    expected = vec![0x7F];
    assert_eq!(vari.into_bytes().unwrap(), expected);
  }

  #[test]
  fn variable_byte_two_into_bytes() {
    let mut vari = DataType::VariableByteInteger(VariableByte::Two(128));
    let mut expected: Vec<u8> = vec![0x80, 0x01];
    assert_eq!(vari.into_bytes().unwrap(), expected);

    vari = DataType::VariableByteInteger(VariableByte::Two(16383));
    expected = vec![0xFF, 0x7F];
    assert_eq!(vari.into_bytes().unwrap(), expected);
  }

  #[test]
  fn variable_byte_three_into_bytes() {
    let mut vari = DataType::VariableByteInteger(VariableByte::Three(16384));
    let mut expected: Vec<u8> = vec![0x80, 0x80, 0x01];
    assert_eq!(vari.into_bytes().unwrap(), expected);

    vari = DataType::VariableByteInteger(VariableByte::Three(2097151));
    expected = vec![0xFF, 0xFF, 0x7F];
    assert_eq!(vari.into_bytes().unwrap(), expected);
  }

  #[test]
  fn variable_byte_four_into_bytes() {
    let mut vari = DataType::VariableByteInteger(VariableByte::Four(2097152));
    let mut expected: Vec<u8> = vec![0x80, 0x80, 0x80, 0x01];
    assert_eq!(vari.into_bytes().unwrap(), expected);

    vari = DataType::VariableByteInteger(VariableByte::Four(268435455));
    expected = vec![0xFF, 0xFF, 0xFF, 0x7F];
    assert_eq!(vari.into_bytes().unwrap(), expected);
  }

  #[test]
  fn variable_byte_into_bytes_error() {
    let vari = DataType::VariableByteInteger(VariableByte::Four(268435456));
    let err = vari.into_bytes().unwrap_err();
    assert_eq!(err, Error::GenerateError);
  }

  #[test]
  fn utf8_string_into_bytes() {
    let value = DataType::Utf8EncodedString("hello world".to_string());
    let expected: Vec<u8> = vec![0, 11, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100];
    assert_eq!(value.into_bytes().unwrap(), expected);
  }

  #[test]
  fn utf8_string_pair_into_bytes() {
    let value = DataType::Utf8StringPair("hello world".to_string(), "foo bar".to_string());
    let expected: Vec<u8> = vec![
      0, 11, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 0, 7, 102, 111, 111, 32, 98, 97,
      114,
    ];
    assert_eq!(value.into_bytes().unwrap(), expected);
  }

  #[test]
  fn binary_data_into_bytes() {
    let data: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
    let value = DataType::BinaryData(data);

    let expected: Vec<u8> = vec![
      0, 10, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
    ];
    assert_eq!(value.into_bytes().unwrap(), expected);
  }

  #[test]
  fn into_bytes_max_length() {
    let data = [0u8; 65536];
    let value = DataType::BinaryData(data.to_vec());
    let err = value.into_bytes().unwrap_err();
    assert_eq!(err, Error::GenerateError);
  }
}
