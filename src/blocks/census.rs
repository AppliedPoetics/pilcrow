use crate::block;
use crate::chain;

pub fn validate() 
  -> Result<(), ()> {
  let chain = chain::get_whole_chain();
  let mut check_block = chain[0].clone();
  for b in 0..chain.len() {
    check_block.calc_hash();
    if chain[b].prev_hash != check_block.hash {
      return Err(());
    }
    check_block = chain[b].clone();
  }
  Ok(())
}