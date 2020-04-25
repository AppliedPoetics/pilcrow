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

fn get_time_now() 
  -> String {
    let system_time = SystemTime::now();
    let date_time: DateTime<Utc> = system_time.into();
    date_time.format("%d-%m-%Y %T").to_string()
  }

fn calculate_hash(block: &Block) 
  -> String {
    let mut hash =  block.num.to_string();
    hash.push_str(&block.time);
    hash.push_str(&block.text);
    hash.push_str(&block.prev_hash);
    hash.to_string()
  }

pub fn make_block(previous: Option<Block>, text: Option<String>, staker: Option<String>) 
  -> Block {
    let block;
    let previous: Block = previous
      .unwrap_or(
        Block {
          num: 0,
          time: get_time_now(),
          text: String::new(),
          hash: String::new(),
          prev_hash: String::new(),
          staker: String::new(),
        }
      );
    block = Block {
      num: 0,
      time: get_time_now(),
      text: text.unwrap_or("".to_string()),
      hash: calculate_hash(&previous),
      prev_hash: previous.hash,
      staker: staker.unwrap_or("".to_string()),
    };
    println!("{:?}",block);
    block
  }