use chrono::DateTime;
use chrono::offset::Utc;
use sha2::{Sha256};
use std::time::{SystemTime};

#[derive(Debug)]
pub struct Block {
  num: u32,
  time: String,
  text: String,
  hash: String,
  prev_hash: String,
  staker: String
}

fn calculate_hash(block: &Block) 
  -> String {
    let mut hash =  block.num.to_string();
    hash.push_str(&block.time);
    hash.push_str(&block.text);
    hash.push_str(&block.prev_hash);
    hash.to_string()
  }

pub fn make_block(previous: Block, text: String, staker: String) 
  -> Block {
  let block;
  let system_time = SystemTime::now();
  let date_time: DateTime<Utc> = system_time.into();
  block = Block {
    num: 0,
    time: date_time.format("%d-%m-%Y %t").to_string(),
    text: text,
    hash: calculate_hash(&previous),
    prev_hash: previous.hash,
    staker: staker,
  };
  println!("{:?}",block);
  block
}