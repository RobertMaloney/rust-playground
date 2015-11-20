use std::env;

struct GameState {
  pub score: u32,
  pub spare_count: u32,
  pub strike_1: u32,
  pub strike_2: u32,
  pub previous_score: u32
}

impl GameState {
  fn new() -> GameState {
    GameState {
      score: 0,
      spare_count: 0,
      strike_1: 0,
      strike_2: 0,
      previous_score: 0
    }
  }

  fn update_state(self, roll: char) -> GameState {
    let (mut sp, mut str1, mut str2) = (self.spare_count, self.strike_1, self.strike_2);
    let ds = match roll {
      '-' => 0,
      '/' => 10 - self.previous_score,
      'X' => 10,
      _ => roll as u32 - '0' as u32
    };

    let weighted_score = ds * [sp, str1, str2].iter()
      .map(|&x| (x > 0) as u32).fold(1, |acc, x| acc + x);

    if sp > 0 { sp -= 1; }
    if str2 > 0 { str2 -= 1; }
    str1 = str2;
    match roll {
      '/' => sp = 1,
      'X' => str2 = 2,
      _ => ()
    }

    GameState {
      score: self.score + weighted_score,
      spare_count: sp,
      strike_1: str1,
      strike_2: str2,
      previous_score: ds
    }
  }
}

fn score(turns: &String) -> u32 {
  let final_game = turns.split_whitespace().fold(GameState::new(), |state, turn| {
    turn.chars().fold(state, |roll_state, roll_score| {
      roll_state.update_state(roll_score)
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
