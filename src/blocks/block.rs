use crate::chain;
use chrono::DateTime;
use chrono::offset::Utc;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use serde_json;
use std::time::{SystemTime};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Block {
  num: usize,
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
    let mut hashed = String::new();
    let mut hasher = Sha256::default();
    let mut hashable =  block.num.to_string();
    hashable.push_str(&block.time);
    hashable.push_str(&block.text);
    hashable.push_str(&block.prev_hash);
    hasher.input(hashable);
    let result = hasher.result();
    for v in result.iter() {
      hashed.push_str(&v.to_string());
    }
    hashed
  }

pub fn make_block(previous: Option<Block>, text: Option<String>, staker: Option<String>) 
  -> Block {
    let block;
    // Get previous block or generate genesis block
    let previous: Block = previous
      .unwrap_or(
        Block {
          num: 0,
          time: get_time_now(),
          text: String::new(),
          hash: String::new(),
          prev_hash: String::new(),
          staker: "Pilcrow Daemon".to_string(),
        }
      );
    let chain_len = chain::get_block_count();
    // Generate next or genesis block
    block = Block {
      num: chain_len,
      time: get_time_now(),
      text: text.unwrap_or("".to_string()),
      hash: calculate_hash(&previous),
      prev_hash: previous.hash,
      staker: staker.unwrap_or("".to_string()),
    };
    block
  }