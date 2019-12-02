use crate::Error;
use std::env;

pub struct Config {
  pub buffer_length: usize,
}

impl Config {
  fn new() -> Result<Self, Error> {
    let conf = Config {
      buffer_length: match env::var("MQTT_PACKET_BUFFER_LENGTH") {
        Ok(val) => val.parse::<usize>()?,
        Err(_e) => 65535,
      },
    };

    return Ok(conf);
  }
}
