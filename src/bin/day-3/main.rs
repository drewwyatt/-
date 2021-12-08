use advent::read_input_for_day;
use std::io;

enum AdventError {
  InvalidBit,
}

struct Position {
  zeroes: usize,
  ones: usize,
}

impl Position {
  pub fn new() -> Position {
    Position { zeroes: 0, ones: 0 }
  }

  pub fn push(&mut self, bit: &char) -> Result<(), AdventError> {
    match bit {
      '0' => {
        self.zeroes += 1;
        Ok(())
      }
      '1' => {
        self.ones += 1;
        Ok(())
      }
      _ => Err(AdventError::InvalidBit),
    }
  }

  pub fn get_gamma(&self) -> &str {
    if self.zeroes > self.ones {
      "0"
    } else {
      "1"
    }
  }

  pub fn get_epsilon(&self) -> &str {
    if self.zeroes > self.ones {
      "1"
    } else {
      "0"
    }
  }
}

struct Diagnostic {
  positions: Vec<Position>,
}

impl Diagnostic {
  pub fn new() -> Diagnostic {
    Diagnostic { positions: vec![] }
  }

  pub fn get_gamma(&self) -> String {
    let mut gamma = "".to_owned();
    for position in self.positions.iter() {
      gamma.push_str(position.get_gamma());
    }

    gamma
  }

  pub fn get_epsilon(&self) -> String {
    let mut epsilon = "".to_owned();
    for position in self.positions.iter() {
      epsilon.push_str(position.get_epsilon());
    }

    epsilon
  }

  pub fn push(&mut self, bit_index: usize, value: &char) -> Result<(), AdventError> {
    match self.positions.get_mut(bit_index) {
      Some(position) => position.push(value),
      None => {
        let mut position = Position::new();
        let res = position.push(value);
        self.positions.push(position);
        res
      }
    }
  }
}

fn part_one(input: &Vec<String>) -> (i64, i64) {
  let mut diagnostic = Diagnostic::new();
  for line in input {
    for (bit_index, value) in line.chars().enumerate() {
      diagnostic.push(bit_index, &value);
    }
  }

  (
    i64::from_str_radix(&diagnostic.get_gamma(), 2).unwrap(),
    i64::from_str_radix(&diagnostic.get_epsilon(), 2).unwrap(),
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_input() -> Vec<String> {
    vec![
      "00100".to_string(),
      "11110".to_string(),
      "10110".to_string(),
      "10111".to_string(),
      "10101".to_string(),
      "01111".to_string(),
      "00111".to_string(),
      "11100".to_string(),
      "10000".to_string(),
      "11001".to_string(),
      "00010".to_string(),
      "01010".to_string(),
    ]
  }

  #[test]
  fn part_one_sample_input() {
    assert_eq!(part_one(&get_input()), (22, 9))
  }
}

fn main() -> io::Result<()> {
  let input = read_input_for_day(3)?;

  let (gamma, epsilon) = part_one(&input);
  println!(
    "[Day 3, Part 1] Gamma: {}, Epsilon: {}, Power Consumption: {}",
    gamma,
    epsilon,
    gamma * epsilon
  );

  Ok(())
}
