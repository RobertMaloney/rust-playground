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
  (0..count).map(|_| {
    let index = (rng.next_u32() as usize) % domain_size;
    let mut s = words.get(index).unwrap().clone();
    s.pop(); // remove CR
    s
  }).collect::<Vec<String>>()
}

fn match_words(a: &String, b: &String) -> (usize, usize) {
  let correct = a.as_bytes().iter().zip(b.as_bytes().iter())
    .fold(0, |acc, (x, y)| acc + (x == y) as usize);
  (correct, b.len())
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
  let mut attempts_left = 4;
  loop {
    print!("Select a word: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut attempt).unwrap();
    attempt.pop();

    // check to make sure written word is an option
    if !my_words.contains(&attempt) {
      println!("That word is invalid.");
      attempt.clear();
      continue;
    }

    let (correct, total) = match_words(&attempt, &answer);
    if correct == total {
      println!("YOU WON!");
      break;
    }
    attempts_left -= 1;
    if attempts_left == 0 {
      println!("YOU LOST!");
      break;
    }
    println!("Match: {}/{}", correct, total);
    println!("Attempts remaining: {}", attempts_left);
    attempt.clear();
  }
}
