use std::env;

fn main() {
  let example = "X -/ X 5- 8/ 9- X 81 1- 4/X";
  let args: Vec<_> = env::args().collect();
  if args.len() != 2 {
    println!("Wong number of arguments.");
    return;
  }
}
