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

fn get_time_now() 
  -> String {
    let system_time = SystemTime::now();
    let date_time: DateTime<Utc> = system_time.into();
    date_time.format("%d-%m-%Y %T").to_string()
  }

pub fn interpret(data: [u8;50]) 
  -> Result<()> {
  let mut msg = convert::data_from_u8(&data);
  let json: Value = serde_json::from_str(&msg).unwrap();
  println!("{:#?}",json);
  let incoming = Incoming {
    purpose: json["purpose"].to_string(),
    time: get_time_now(),
    staker: json["staker"].to_string(),
  };
  Ok(())
}