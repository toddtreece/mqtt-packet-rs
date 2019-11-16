use crate::DataType;
use crate::PacketType;
use crate::Property;

pub struct ControlPacket {
  pub packet_type: PacketType,
  pub flags: DataType,
  pub properties: Property,
  pub identifier: DataType,
  pub payload: DataType,
}
