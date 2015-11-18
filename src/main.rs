extern crate rand;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use rand::{thread_rng, Rng};

fn grab_words(len: usize, count: usize) -> Vec<String> {
  let mut rng = thread_rng();
  let file_name = "words.txt";
  let f = File::open(file_name).unwrap();
  let br = BufReader::new(f);
  let words: Vec<_> = br.lines().filter(|x| x.as_ref().unwrap().len() == len + 1).map(|x| x.unwrap()).collect();
  let domain_size = words.len();
  let mut v: Vec<String> = vec![];
  for _ in 0..count {
    let index = (rng.next_u32() as usize) % domain_size;
    let s = words.get(index).unwrap();
    v.push(s.clone());
  }
  v
}

fn main() {
  let my_words: Vec<String> = grab_words(5, 20);
  for word in my_words {
    println!("{}", word);
  }
}
