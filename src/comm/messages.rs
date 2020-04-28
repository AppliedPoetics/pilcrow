use crate::clock;
use crate::convert;
use crate::router;
use chrono::DateTime;
use chrono::offset::Utc;
use serde::{Serialize, Deserialize};
use serde_json::*;
use std::time::{SystemTime};

pub struct Incoming {
  pub purpose: String,
  pub time: String,
  pub staker: String,
}

pub fn interpret(data: [u8;50])
  -> Result<()> {
    let mut msg = convert::data_from_u8(&data);
    let json: Value = serde_json::from_str(&msg)?;
    let incoming = Incoming {
      purpose: json["purpose"].to_string(),
      time: clock::get_time_now(),
      staker: json["staker"].to_string(),
    };
    router::route(&incoming);
    Ok(())
  }