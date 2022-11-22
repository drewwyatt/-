mod models;
use std::io;

use advent::read_input_for_day;
use models::school::School;

fn simulate(mut school: School, days: i32) -> usize {
  for _ in 0..days {
    school.tick()
  }

  school.size()
}

fn main() -> io::Result<()> {
  let school = read_input_for_day::<School>(6)?.get(0).unwrap().to_owned();
  let size = simulate(school, 80);

  println!("Part-1: After 80 days there are {} lantern fish.", size);
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
    let school = get_initial_state();
    let size = simulate(school, 80);

    assert_eq!(size, 5934);
  }

  #[test]
  fn part_two() {}
}
