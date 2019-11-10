use std::cmp;
use std::ops::{Index};
use super::constants::MAX_CHARS;


pub struct Word {
  chars: [char; MAX_CHARS],
  pub len: usize,
}


impl Word {
  pub fn new() -> Word {
      let chars = ['\0'; MAX_CHARS];
      let len = 0;
      Word { chars, len }
  }

  pub fn write(&mut self, s: &str) -> () {
      self.len = cmp::min(s.len(), MAX_CHARS);
      for (i, c) in s.chars().take(self.len).enumerate() {
          self.chars[i] = c;
      }
  }
}


impl Index<usize> for Word {
  type Output = char;
  fn index(&self, i: usize) -> &Self::Output {
      if i >= self.len {
          panic!("Char index out of boundary");
      }
      &self.chars[i]
  }
}