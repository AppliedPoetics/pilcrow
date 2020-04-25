use crate::block;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
  static ref BLOCKCHAIN: Mutex<Vec<block::Block>> = {
    let chain = Vec::new();
    Mutex::new(chain)
  };
}

pub fn add_block(block: block::Block) 
  -> Result<(), ()> {
    let mut chain = BLOCKCHAIN
      .lock()
      .unwrap();
    chain.push(block);
    println!("{:?}",chain);
    Ok(())
}