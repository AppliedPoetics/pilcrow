use crate::block;

use serde::{Serialize, Deserialize};
use serde_json;

pub unsafe fn block_as_u8_slice<T: Sized>(p: &T)
  -> &[u8] {
    ::std::slice::from_raw_parts(
      (p as *const T) as *const u8,
      ::std::mem::size_of::<T>(),
    )
  }
  
pub fn block_to_json(block: &block::Block)
  -> String {
    let json = serde_json::to_string(block);
    json.unwrap()
  }