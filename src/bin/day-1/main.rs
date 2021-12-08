mod input;
use input::read;
use std::io;

fn part_one(input: &Vec<i64>) -> i64 {
  let mut number_of_increases = 0;
  let mut previous = &std::i64::MAX;
  for reading in input {
    if reading > previous {
      number_of_increases += 1;
    }

    previous = reading;
  }

  number_of_increases
}

fn sum_for_index(input: &Vec<i64>, index: usize) -> i64 {
  input[index] + input[index + 1] + input[index + 2]
}

fn part_two(input: &Vec<i64>) -> usize {
  let mut number_of_increases = 0;
  let mut previous = std::i64::MAX;
  let mut index = 0;

  while index < input.len() - 2 {
    let sum = sum_for_index(input, index);
    if sum > previous {
      number_of_increases += 1;
    }

    previous = sum;
    index += 1;
  }

  number_of_increases
}

#[cfg(test)]
mod tests {
  use super::*;
  const INPUT: [i64; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

  #[test]
  fn part_one_sample_input() {
    assert_eq!(part_one(&INPUT.to_vec()), 7);
  }

  #[test]
  fn part_two_sample_input() {
    assert_eq!(part_two(&INPUT.to_vec()), 5);
  }
}

fn main() -> io::Result<()> {
  let input = read()?;

  println!("[Day 1, Part 1] There were {} increases.", part_one(&input));
  println!("[Day 1, Part 2] There were {} increases.", part_two(&input));

  Ok(())
}
