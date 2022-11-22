use super::fish::Fish;
use std::str::FromStr;

#[derive(Debug)]
pub enum AdventError {}

pub struct School {
  fish: Vec<Fish>,
}

impl School {
  pub fn size(&self) -> usize {
    self.fish.len()
  }

  pub fn tick(&mut self) {
    let mut babies = vec![];
    for next_fish in self.fish.iter_mut() {
      match next_fish.tick() {
        Some(baby) => babies.push(baby),
        _ => (),
      }
    }

    if !babies.is_empty() {
      self.fish.append(&mut babies);
    }
  }
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
