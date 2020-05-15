use crate::block;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
  static ref BLOCKCHAIN: Mutex<Vec<block::Block>> = {
    let chain = Vec::new();
    Mutex::new(chain)
  };
}

pub fn get_whole_chain()
  -> Vec<block::Block> {
    let chain = &BLOCKCHAIN
      .lock()
      .unwrap();
    chain.to_vec()
  }

pub fn get_latest_block()
  -> block::Block {
    let chain = &BLOCKCHAIN;
    let owned = chain
      .lock()
      .unwrap();
    let last_block = owned
      .last()
      .unwrap()
      .clone();
    last_block
  }
  
pub fn get_block_count()
  -> usize {
    let chain = &BLOCKCHAIN;
    let owned = chain
      .lock()
      .unwrap();
    let count: usize = owned.len();
    count
  }

pub fn add_block(block: block::Block) 
  -> Result<(), ()> {
    let mut chain = BLOCKCHAIN
      .lock()
      .unwrap();
    chain.push(block);
    Ok(())
  }