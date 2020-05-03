use crate::block;
use crate::chain;
use crate::clock;
use serde::{Serialize, Deserialize};

pub struct Incoming {
  pub purpose: String,
  pub time: String,
  pub sender: String,
}

pub struct Contribution {
  pub text: String,
  pub sender: String,
}

pub struct Confirmation {
  pub response: block::Block,
}

pub struct Synchronization {
  pub response: Vec<block::Block>
}