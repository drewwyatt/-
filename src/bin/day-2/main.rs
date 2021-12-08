#[macro_use]
extern crate lazy_static;
mod input;

use advent::read_input_for_day;
use input::{Command, Direction};
use std::io;

fn part_one(course: &Vec<Command>) -> (usize, usize) {
  let mut position = 0;
  let mut depth = 0;

  for command in course {
    match command.direction {
      Direction::Forward => {
        position += command.delta;
      }
      Direction::Down => {
        depth += command.delta;
      }
      Direction::Up => {
        depth -= command.delta;
      }
    }
  }

  (position, depth)
}

fn part_two(course: &Vec<Command>) -> (usize, usize) {
  let mut position = 0;
  let mut depth = 0;

  for command in course {
    match command.direction {
      Direction::Forward => {
        position += command.delta;
      }
      Direction::Down => {
        depth += command.delta;
      }
      Direction::Up => {
        depth -= command.delta;
      }
    }
  }

  (position, depth)
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::str::FromStr;

  fn get_input() -> Vec<Command> {
    vec![
      Command::from_str("forward 5").unwrap(),
      Command::from_str("down 5").unwrap(),
      Command::from_str("forward 8").unwrap(),
      Command::from_str("up 3").unwrap(),
      Command::from_str("down 8").unwrap(),
      Command::from_str("forward 2").unwrap(),
    ]
  }

  #[test]
  fn part_one_sample_input() {
    assert_eq!(part_one(&get_input()), (15, 10));
  }

  #[test]
  fn part_two_sample_input() {
    assert_eq!(part_two(&get_input()), (15, 60));
  }
}

fn main() -> io::Result<()> {
  let input = read_input_for_day::<Command>(2)?;

  let (position, depth) = part_one(&input);
  println!(
    "[Day 2, Part 1] Position: {}, Depth: {}, Product: {}.",
    position,
    depth,
    position * depth
  );

  Ok(())
}
