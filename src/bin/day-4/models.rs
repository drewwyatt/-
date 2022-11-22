struct Board {
  spaces: Vec<Space>,
}

const NUM_COLS: usize = 5;
const NUM_ROWS: usize = 5;

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

  pub fn is_winner(&self) -> bool {
    self
      .rows()
      .iter()
      .any(|slice| slice.iter().all(|space| space.marked))
      || self
        .columns()
        .iter_mut() // weird that this has to be mutable?
        .any(|col| col.all(|space| space.marked))
  }

  fn rows(&self) -> Vec<&[Space]> {
    let mut result: Vec<&[Space]> = vec![];
    for i in 0..NUM_ROWS {
      let start = i * NUM_COLS;
      let end = start + NUM_COLS;
      result.push(&self.spaces[start..end]);
    }

    result
  }

  fn columns(&self) -> Vec<impl Iterator<Item = &Space>> {
    let mut result = vec![];
    for i in 0..NUM_COLS {
      result.push(self.get_column(i));
    }

    result
  }

  fn get_column(&self, col_num: usize) -> impl Iterator<Item = &Space> {
    self.spaces.iter().skip(col_num).step_by(NUM_COLS)
  }
}

#[derive(Debug)]
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
  fn make_board_1() -> Board {
    Board::new(vec![
      22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15, 19,
    ])
  }

  fn make_board_2() -> Board {
    Board::new(vec![
      3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16, 12, 6,
    ])
  }

  fn make_board_3() -> Board {
    Board::new(vec![
      14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3, 7,
    ])
  }

  fn make_boards() -> Vec<Board> {
    vec![make_board_1(), make_board_2(), make_board_3()]
  }

  #[test]
  fn non_winners() {
    let mut board = make_board_1();
    assert_eq!(board.is_winner(), false);

    board.check(22);
    board.check(13);
    board.check(17);
    board.check(11);

    board.check(8);
    board.check(21);
    board.check(6);
    assert_eq!(board.is_winner(), false);
  }

  #[test]
  fn can_recognize_winning_row() {
    let mut boards = make_boards();
    let spaces = vec![
      vec![22, 13, 17, 11, 0],
      vec![9, 18, 13, 17, 5],
      vec![18, 8, 23, 26, 20],
    ];

    for (index, group) in spaces.iter().enumerate() {
      for space in group {
        boards[index].check(*space);
      }
    }

    for board in boards {
      assert_eq!(board.is_winner(), true);
    }
  }

  #[test]
  fn can_recognize_winning_col() {
    let mut boards = make_boards();
    let spaces = vec![
      vec![22, 8, 21, 6, 1],
      vec![15, 18, 8, 11, 21],
      vec![17, 15, 23, 13, 12],
    ];

    for (index, group) in spaces.iter().enumerate() {
      for space in group {
        boards[index].check(*space);
      }
    }

    for board in boards {
      assert_eq!(board.is_winner(), true);
    }
  }
}
