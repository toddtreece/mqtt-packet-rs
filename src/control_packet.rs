use crate::data_type::DataType;
use crate::packet_type::PacketType;
use crate::property::Property;

pub struct ControlPacket {
  pub packet_type: PacketType,
  pub flags: DataType,
  pub properties: Property,
  pub identifier: DataType,
  pub payload: DataType,
}
