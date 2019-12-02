//! A [MQTT v5.0][mqtt] packet parser and generator. This crate is unstable and under active development.
//!
//! [mqtt]: https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html

mod config;
mod data_type;
mod error;
mod macros;
mod packet_type;
mod property;

pub use config::Config;
pub use data_type::{DataType, VariableByte};
pub use error::Error;
pub use packet_type::PacketType;
pub use property::{Identifier, Property};
