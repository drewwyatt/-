pub struct Fish {
  timer: i8,
}

impl Fish {
  pub fn from(timer: i8) -> Self {
    Fish { timer }
  }

  pub fn new() -> Self {
    Fish { timer: 8 }
  }

  pub fn tick(&mut self) -> Option<Self> {
    if self.timer == 0 {
      self.timer = 6;
      return Some(Fish::new());
    }

    self.timer -= 1;
    None
  }
}
