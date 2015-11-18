extern crate rand;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use rand::{thread_rng, Rng};

fn grab_words(len: usize, count: usize) -> Vec<String> {
  let mut rng = thread_rng();
  let file_name = "words.txt";
  let f = File::open(file_name).unwrap();
  let br = BufReader::new(f);
  let words: Vec<_> = br.lines().map(|x| x.unwrap()).filter(|x| x.len() == len + 1).collect();
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
  let mut difficulty = String::new();
  print!("Select your difficulty (easy/medium/hard): ");
  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut difficulty).unwrap();
  difficulty.pop(); // remove newline
  let my_words: Vec<String> = match difficulty.as_ref() {
    "easy" => grab_words(4, 7),
    "medium" => grab_words(5, 13),
    "hard" => grab_words(7, 16),
    _ => panic!("Invalid difficulty!")
  };
  for word in my_words {
    println!("{}", word);
  }
}
