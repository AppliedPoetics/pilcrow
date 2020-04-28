use chrono::DateTime;
use chrono::offset::Utc;
use std::time::{SystemTime};

pub fn get_time_now() 
  -> String {
    let system_time = SystemTime::now();
    let date_time: DateTime<Utc> = system_time.into();
    date_time.format("%d-%m-%Y %T").to_string()
  }