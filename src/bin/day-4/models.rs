struct Board {
  spaces: Vec<Space>,
}

impl Board {
  pub fn new(spaces: Vec<usize>) -> Board {
    Board {
      spaces: spaces.iter().map(|s| Space::new(s.to_owned())).collect(),
    }
  }

  pub fn check(&mut self, number: usize) {
    for space in &mut self.spaces {
      if space.number == number {
        space.mark();
      }
    }
  }

  pub fn has_winner(&self) -> bool {
    self
      .rows()
      .iter()
      .any(|slice| slice.iter().all(|space| space.marked))
  }

  fn rows(&self) -> Vec<&[Space]> {
    let mut result: Vec<&[Space]> = vec![];
    for i in 0..4 {
      let start = i * 5;
      let end = start + 4;
      result.push(&self.spaces[start..end]);
    }

    result
  }
}

struct Space {
  pub marked: bool,
  number: usize,
}

impl Space {
  pub fn new(number: usize) -> Space {
    Space {
      marked: false,
      number,
    }
  }

  pub fn mark(&mut self) {
    self.marked = true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  fn get_boards() -> Vec<Board> {
    vec![
      Board::new(vec![
        22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15, 19,
      ]),
      Board::new(vec![
        3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16, 12, 6,
      ]),
      Board::new(vec![
        14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3, 7,
      ]),
    ]
  }
}
