use crate::Error;
use crate::PacketType;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq)]
pub struct GenericFlags(bool, bool, bool, bool);

#[derive(Debug, PartialEq, Eq)]
pub struct PublishFlags {
  retain: bool,
  qos: u8,
  dup: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Flags {
  Generic(GenericFlags),
  Publish(PublishFlags),
}

impl Flags {
  pub fn new(header: u8) -> Result<Self, Error> {
    let packet_type = PacketType::try_from((header & 0xF0) >> 4)?;

    let generic_flags = Self::Generic(GenericFlags(
      (header & 0x01) == 0x01,
      (header & 0x02) == 0x02,
      (header & 0x04) == 0x04,
      (header & 0x08) == 0x08,
    ));

    match packet_type {
      PacketType::PUBLISH => {
        let qos = (header & 0x06) >> 1;

        // A PUBLISH Packet MUST NOT have both QoS bits set to 1 [MQTT-3.3.1-4].
        // If a Server or Client receives a PUBLISH packet which has both QoS
        // bits set to 1 it is a Malformed Packet. Use DISCONNECT with Reason
        // Code 0x81 (Malformed Packet) as described in section 4.13.
        if qos > 2 {
          return Err(Error::MalformdedPacket);
        }

        let flags = Self::Publish(PublishFlags {
          retain: (header & 0x01) == 0x01,
          qos,
          dup: (header & 0x08) == 0x08,
        });

        Ok(flags)
      }
      PacketType::PUBREL | PacketType::SUBSCRIBE | PacketType::UNSUBSCRIBE => {
        // Where a flag bit is marked as “Reserved”, it is reserved for future
        // use and MUST be set to the value listed [MQTT-2.1.3-1]. If invalid
        // flags are received it is a Malformed Packet. Refer to section 4.13
        // for details about handling errors.
        if (header & 0x0F) == 0x02 {
          Ok(generic_flags)
        } else {
          Err(Error::MalformdedPacket)
        }
      }
      _ => Ok(generic_flags),
    }
  }

  /// Convert Flag variants into u8.
  pub fn to_u8(&self) -> Result<u8, Error> {
    let mut flag: u8 = 0x00;

    match self {
      Flags::Publish(value) => {
        flag |= value.qos << 1;
        if value.retain {
          flag |= 0x01
        }
        if value.dup {
          flag |= 0x08
        }
      }
      Flags::Generic(value) => {
        if value.0 {
          flag |= 0x01;
        }
        if value.1 {
          flag |= 0x02;
        }
        if value.2 {
          flag |= 0x04;
        }
        if value.3 {
          flag |= 0x08;
        }
      }
    };

    Ok(flag)
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn publish() {
    let fixed_header: u8 = 0x3D;
    let flag_type = super::Flags::new(fixed_header);
    assert_eq!(
      flag_type.unwrap(),
      super::Flags::Publish(super::PublishFlags {
        retain: true,
        qos: 2,
        dup: true
      })
    );
  }

  #[test]
  fn publish_qos_error() {
    let fixed_header: u8 = 0x3F;
    let flag_type = super::Flags::new(fixed_header);
    assert_eq!(flag_type.unwrap_err(), crate::Error::MalformdedPacket);
  }

  #[test]
  fn generic_connect() {
    let fixed_header: u8 = 0x1F;
    let flag_type = super::Flags::new(fixed_header);
    assert_eq!(
      flag_type.unwrap(),
      super::Flags::Generic(super::GenericFlags(true, true, true, true))
    );
  }

  #[test]
  fn generic_connack() {
    let fixed_header: u8 = 0x21;
    let flag_type = super::Flags::new(fixed_header);
    assert_eq!(
      flag_type.unwrap(),
      super::Flags::Generic(super::GenericFlags(true, false, false, false))
    );
  }

  #[test]
  fn generic_pubrel() {
    let fixed_header: u8 = 0x62;
    let flag_type = super::Flags::new(fixed_header);
    assert_eq!(
      flag_type.unwrap(),
      super::Flags::Generic(super::GenericFlags(false, true, false, false))
    );
  }

  #[test]
  fn generic_subscribe() {
    let fixed_header: u8 = 0x82;
    let flag_type = super::Flags::new(fixed_header);
    assert_eq!(
      flag_type.unwrap(),
      super::Flags::Generic(super::GenericFlags(false, true, false, false))
    );
  }

  #[test]
  fn generic_unsubscribe() {
    let fixed_header: u8 = 0xA2;
    let flag_type = super::Flags::new(fixed_header);
    assert_eq!(
      flag_type.unwrap(),
      super::Flags::Generic(super::GenericFlags(false, true, false, false))
    );
  }

  #[test]
  fn reserved_error() {
    let fixed_header: u8 = 0xAF;
    let flag_type = super::Flags::new(fixed_header);
    assert_eq!(flag_type.unwrap_err(), crate::Error::MalformdedPacket);
  }

  #[test]
  fn publish_truthy_to_u8() {
    let flag_type = super::Flags::Publish(super::PublishFlags {
      retain: true,
      qos: 2,
      dup: true,
    });
    assert_eq!(flag_type.to_u8().unwrap(), 0x0D);
  }

  #[test]
  fn publish_falsy_to_u8() {
    let flag_type = super::Flags::Publish(super::PublishFlags {
      retain: false,
      qos: 1,
      dup: false,
    });
    assert_eq!(flag_type.to_u8().unwrap(), 0x02);
  }

  #[test]
  fn generic_one_to_u8() {
    let flag_type = super::Flags::Generic(super::GenericFlags(true, false, false, false));
    assert_eq!(flag_type.to_u8().unwrap(), 0x01);
  }

  #[test]
  fn generic_two_to_u8() {
    let flag_type = super::Flags::Generic(super::GenericFlags(true, true, false, false));
    assert_eq!(flag_type.to_u8().unwrap(), 0x03);
  }

  #[test]
  fn generic_three_to_u8() {
    let flag_type = super::Flags::Generic(super::GenericFlags(true, true, true, false));
    assert_eq!(flag_type.to_u8().unwrap(), 0x07);
  }

  #[test]
  fn generic_four_to_u8() {
    let flag_type = super::Flags::Generic(super::GenericFlags(true, true, true, true));
    assert_eq!(flag_type.to_u8().unwrap(), 0x0F);
  }
}
