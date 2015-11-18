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

fn match_words(a: &String, b: &String) -> (usize, usize) {
  let mut correct: usize = 0;
  let mut total: usize = 0;
  for (x,y) in a.as_bytes().iter().zip(b.as_bytes().iter()) {
    if x == y { correct += 1; }
    total += 1;
  };
  (correct, total)
}

fn main() {
  let mut rng = thread_rng();

  // request difficulty from the user
  let mut difficulty = String::new();
  print!("Select your difficulty (easy/medium/hard): ");
  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut difficulty).unwrap();
  difficulty.pop(); // remove newline

  // generate hack words depending on the difficulty
  let my_words: Vec<String> = match difficulty.as_ref() {
    "easy" => grab_words(4, 7),
    "medium" => grab_words(5, 13),
    "hard" => grab_words(7, 16),
    _ => panic!("Invalid difficulty!")
  };

  // display potential answers
  for word in my_words.iter() {
    println!("{}", word);
  }

  // select one to be the answer
  let answer = my_words.get((rng.next_u32() as usize) % my_words.len()).unwrap();
  println!("ANSWER: {}", answer);

  // loop as user attempts to find the correct word
  let mut attempt = String::new();
  loop {
    print!("Select a word: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut attempt).unwrap();
    attempt.pop();
    let (correct, total) = match_words(&attempt, &answer);
    if correct == total { break; }
    println!("Match: {}/{}", correct, total);
    attempt.clear();
  }
}
