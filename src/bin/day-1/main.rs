fn part_one(input: Vec<i64>) -> i64 {
  let mut number_of_increases = 0;
  let mut previous = std::i64::MAX;
  for reading in input {
    if reading > previous {
      number_of_increases += 1;
    }

    previous = reading;
  }

  number_of_increases
}

#[cfg(test)]
mod tests {
  use super::*;
  const INPUT: [i64; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

  #[test]
  fn part_one_sample_input() {
    assert_eq!(part_one(INPUT.to_vec()), 7);
  }
}

fn main() {
  println!("Hello, day 1!");
}
