use std::env;

struct GameState {
  pub score: u32,
  pub spare_count: u32,
  pub strike_1: u32,
  pub strike_2: u32
}

impl GameState {
  fn new() -> GameState {
    GameState {
      score: 0,
      spare_count: 0,
      strike_1: 0,
      strike_2: 0
    }
  }
}

fn score(turns: &String) -> u32 {
  let final_game = turns.split_whitespace().fold(GameState::new(), |state, turn| {
    turn.chars().fold(state, |roll_state, roll_score| {
      match roll_score {
        '-' => (),
        '/' => (),
        'X' => (),
        _ => ()
      }
      roll_state
    })
  });
  final_game.score
}

// X  X  X  X  X   X   X   X   X   XXX
// 10 30 60 90 120 150 180 210 240 270 300

fn main() {
  // let example = "X -/ X 5- 8/ 9- X 81 1- 4/X";
  let args: Vec<_> = env::args().collect();
  if args.len() != 2 {
    println!("Wong number of arguments.");
    return;
  }

  println!("SCORE: {}", score(args.get(1).unwrap()));
}
