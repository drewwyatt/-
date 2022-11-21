use advent::parse_from_named_captures_or;
use regex::Regex;
use std::{str::FromStr, collections::HashMap};

#[derive(Clone, Copy, Debug)]
pub enum AdventError {
  InvalidRegex,
}

type Coord = (i32, i32);

pub struct Line {
  pub start: Coord,
  pub end: Coord,
}

impl Line {
  pub fn is_horizontal(&self) -> bool {
    self.start.0 == self.end.0
  }

  pub fn is_vertical(&self) -> bool {
    !&self.is_horizontal()
  }
}

impl FromStr for Line {
  type Err = AdventError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"^(?P<x1>\d+),(?P<y1>\d+)\W->\W(?P<x2>\d+),(?P<y2>\d+)$").unwrap();
    }

    let captures = RE.captures(s).ok_or(AdventError::InvalidRegex)?;

    let x1 = parse_from_named_captures_or(&captures, "x1", AdventError::InvalidRegex)?;
    let y1 = parse_from_named_captures_or(&captures, "y1", AdventError::InvalidRegex)?;

    let x2 = parse_from_named_captures_or(&captures, "x2", AdventError::InvalidRegex)?;
    let y2 = parse_from_named_captures_or(&captures, "y2", AdventError::InvalidRegex)?;

    Ok(Line { start: (x1, y1), end: (x2, y2) })
  }
}

pub struct Graph {
  values: HashMap<Coord, i32>
}

impl Graph {
  pub fn chart(&self, line: Line) {
    if line.is_horizontal() {
      let x = line.start.0;
      for y in line.start.1..line.end.1 {
        self.mark_coordinate(x, y);
      }
    } else {
      let y = line.start.1;
      for x in line.start.0..line.end.0 {
        self.mark_coordinate(x, y);
      }
    }
  }

  fn mark_coordinate(&self, x: i32, y: i32) {

  }
}