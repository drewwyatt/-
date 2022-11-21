use advent::parse_from_named_captures_or;
use regex::Regex;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
pub enum AdventError {
  InvalidRegex,
}

pub struct Line {
  pub start: (i32, i32),
  pub end: (i32, i32),
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