use std::{collections::HashMap};
use super::coord::Coord;
use super::line::Line;

pub struct Graph {
  chart_diagonals: bool,
  values: HashMap<Coord, i32>,
}

impl Graph {
  pub fn new(chart_diagonals: bool) -> Self {
      Graph {
          chart_diagonals,
          values: HashMap::new(),
      }
  }

  pub fn n_overlapping_points(&self) -> usize {
      self.values
          .iter()
          .map(|(_, v)| v)
          .filter(|v| v > &&1)
          .collect::<Vec<&i32>>()
          .len()
  }

  pub fn chart(&mut self, line: Line) {
      for coord in line.segments() {
          self.plot_coordinate(coord);
      }
  }

  fn plot_coordinate(&mut self, key: Coord) {
      if self.values.contains_key(&key) {
          *self.values.get_mut(&key).unwrap() += 1;
      } else {
          self.values.insert(key, 1);
      }
  }
}