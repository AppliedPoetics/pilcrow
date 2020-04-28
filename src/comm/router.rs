use crate::messages;
use serde::{Serialize, Deserialize};
use serde_json::*;

pub fn route(incoming: &messages::Incoming) {
    let purpose = &incoming.purpose;
    println!("{}",purpose);
  }