pub mod packet_type;
pub mod property;

use crate::data_type::DataType;
use packet_type::PacketType;
use property::Property;

pub struct ControlPacket {
  pub packet_type: PacketType,
  pub flags: DataType,
  pub properties: Property,
  pub identifier: DataType,
  pub payload: DataType,
}
