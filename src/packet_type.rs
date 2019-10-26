use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

/**
 * 2.1.2 MQTT Control Packet type
 * https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901022
 * Position: byte 1, bits 7-4.
 * Represented as a 4-bit unsigned value.
 */
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, FromPrimitive, ToPrimitive)]
pub enum PacketType {
    CONNECT = 1,
    CONNACK,
    PUBLISH,
    PUBACK,
    PUBREC,
    PUBREL,
    PUBCOMP,
    SUBSCRIBE,
    SUBACK,
    UNSUBSCRIBE,
    UNSUBACK,
    PINGREQ,
    PINGRESP,
    DISCONNECT,
    AUTH,
}
