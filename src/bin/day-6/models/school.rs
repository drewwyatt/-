use super::fish::Fish;
use std::str::FromStr;

enum AdventError {
  InvalidInput,
}

struct School {
  fish: Vec<Fish>,
}

impl FromStr for School {
  type Err = AdventError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let fish = s
      .split(",")
      .map(|s| Fish::from(s.parse::<i8>().unwrap()))
      .collect::<Vec<Fish>>();

    Ok(School { fish })
  }
}
