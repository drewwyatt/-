use advent::parse_from_captures_or;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum AdventError {
  InvalidRegex,
  UnrecognizedDirection,
}

#[derive(Debug)]
pub enum Direction {
  Forward,
  Up,
  Down,
}

impl FromStr for Direction {
  type Err = AdventError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "forward" => Ok(Direction::Forward),
      "up" => Ok(Direction::Up),
      "down" => Ok(Direction::Down),
      _ => Err(AdventError::UnrecognizedDirection),
    }
  }
}

#[derive(Debug)]
pub struct Command {
  delta: usize,
  direction: Direction,
}

impl FromStr for Command {
  type Err = AdventError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"^(forward|down|up) (\d)$").unwrap();
    }

    let captures = RE.captures(s).ok_or(AdventError::InvalidRegex)?;
    let direction =
      parse_from_captures_or::<Direction, AdventError>(&captures, 1, AdventError::InvalidRegex)?;
    let delta =
      parse_from_captures_or::<usize, AdventError>(&captures, 2, AdventError::InvalidRegex)?;

    Ok(Command { delta, direction })
  }
}
