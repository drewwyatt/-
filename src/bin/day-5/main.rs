#[macro_use]
extern crate lazy_static;

mod input;
use std::str::FromStr;

use input::Line;

fn main() {}

#[test]
fn test_one() {
  let lines: Vec<Line> = vec![
    "0,9 -> 5,9",
    "8,0 -> 0,8",
    "9,4 -> 3,4",
    "2,2 -> 2,1",
    "7,0 -> 7,4",
    "6,4 -> 2,0",
    "0,9 -> 2,9",
    "3,4 -> 1,4",
    "0,0 -> 8,8",
    "5,5 -> 8,2"
  ].into_iter().map(|s| Line::from_str(s).unwrap()).collect();

  assert_eq!(lines[0].start.0, 0);
  assert_eq!(lines[0].start.1, 9);

  assert_eq!(lines[1].end.0, 0);
  assert_eq!(lines[1].end.1, 8);
}
