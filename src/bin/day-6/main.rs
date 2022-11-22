mod models;
use std::io;

use models::school::School;

fn main() -> io::Result<()> {
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::str::FromStr;

  fn get_initial_state() -> School {
    School::from_str("3,4,3,1,2").unwrap()
  }

  #[test]
  fn part_one() {
    let mut school = get_initial_state();
    for _ in 0..80 {
      school.tick();
    }

    assert_eq!(school.size(), 5934);
  }

  #[test]
  fn part_two() {}
}
