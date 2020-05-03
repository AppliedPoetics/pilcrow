use crate::clock;
use crate::convert;
use crate::packets;
use crate::router;
use chrono::DateTime;
use chrono::offset::Utc;
use serde::{Serialize, Deserialize};
use serde_json::*;
use std::time::{SystemTime};

pub fn interpret(data: [u8;50])
  -> Result<String> {
    let mut msg = convert::data_from_u8(&data);
    let json: Value = serde_json::from_str(&msg)?;
    let incoming = packets::Incoming {
      purpose: json["purpose"].to_string(),
      time: clock::get_time_now(),
      sender: json["sender"].to_string(),
    };
    let response = router::route(&incoming);
    Ok(String::from(""))
  }