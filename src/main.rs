use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn grab_words(count: usize, len: usize) -> Vec<String> {
  let mut v = vec![];
  let file_name = "words.txt";
  let f = File::open(file_name).unwrap();
  let mut br = BufReader::new(f);
  while v.len() < count {
    let mut s = String::new();
    br.read_line(&mut s).unwrap();
    s.pop(); // remove the newline from the end
    v.push(s);
  }
  v
}

fn main() {
  let my_words: Vec<String> = grab_words(3,0);
  for word in my_words {
    println!("{}", word);
  }
}
