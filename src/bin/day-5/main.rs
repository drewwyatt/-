#[macro_use]
extern crate lazy_static;
mod input;

use advent::read_input_for_day;
use input::{Graph, Line};
use std::io;

fn main() -> io::Result<()> {
  let input = read_input_for_day::<Line>(5)?;
  let mut graph = Graph::new();
  for line in input {
    graph.chart(line);
  }

  println!("At least two lines overlap in {} points.", graph.n_overlapping_points());
  Ok(())
}


#[cfg(test)]
mod tests {
  use super::*;
  use std::str::FromStr;

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

    let mut graph = Graph::new();
    for line in lines {
      graph.chart(line);
    }

    assert_eq!(graph.n_overlapping_points(), 5)
  }
}