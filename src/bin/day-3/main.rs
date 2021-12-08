use advent::read_input_for_day;
use std::io;

fn part_one(input: &Vec<String>) -> (usize, usize) {
  let mut gamma = 0;
  let mut epsilon = 0;

  (gamma, epsilon)
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
