use crate::convert;
use chrono::DateTime;
use chrono::offset::Utc;
use serde::{Serialize, Deserialize};
use serde_json::*;
use std::time::{SystemTime};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Incoming {
  purpose: String,
  time: String,
  staker: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Logon {
  time: String,
  staker: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Contribution {
  time: String,
  text: String,
  staker: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Query {
  time: String,
  staker: String,
}

pub fn interpret(data: [u8;50]) 
  -> Result<()> {
  let msg = convert::data_from_u8(&data);
  println!("{}",msg);
  let json: Value = serde_json::from_str(&msg)?;
  let incoming = Incoming {
    purpose: json["purpose"].to_string(),
    time: "".to_string(),
    staker: json["staker"].to_string(),
  };
  Ok(())
}