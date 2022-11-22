use advent::parse_from_named_captures_or;
use regex::Regex;
use std::str::FromStr;

use super::coord::Coord;

#[derive(Clone, Copy, Debug)]
pub enum AdventError {
    InvalidRegex,
}

pub struct Line {
    pub start: Coord,
    pub end: Coord,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_vertical(&self) -> bool {
        self.start.y == self.end.y
    }

    fn get_x_calculator(&self) -> impl Fn(i32) -> i32 + '_ {
        |n| {
            if self.start.x < self.end.x {
                return self.start.x + n;
            } else {
                return self.start.x - n;
            }
        }
    }

    fn get_y_calculator(&self) -> impl Fn(i32) -> i32 + '_ {
        |n| {
            if self.start.y < self.end.y {
                return self.start.y + n;
            } else {
                return self.start.y - n;
            }
        }
    }

    pub fn segments(&self, chart_diagonals: bool) -> Vec<Coord> {
        let mut segments: Vec<Coord> = vec![];
        if self.is_horizontal() {
            let start = if self.start.y < self.end.y {
                self.start.y
            } else {
                self.end.y
            };
            let end = if self.start.y < self.end.y {
                self.end.y
            } else {
                self.start.y
            };

            for y in start..=end {
                segments.push(Coord::new(self.start.x, y))
            }
        } else if self.is_vertical() {
            let start = if self.start.x < self.end.x {
                self.start.x
            } else {
                self.end.x
            };
            let end = if self.start.x < self.end.x {
                self.end.x
            } else {
                self.start.x
            };

            for x in start..=end {
                segments.push(Coord::new(x, self.start.y))
            }
        } else if chart_diagonals {
            let calc_x = self.get_x_calculator();
            let calc_y = self.get_y_calculator();
            let mut x = self.start.x;
            let mut y = self.start.y;
            let mut diff = 0;

            while x != self.end.x && y != self.end.y {
                x = calc_x(diff);
                y = calc_y(diff);
                segments.push(Coord::new(x, y));

                diff += 1;
            }
        }

        segments
    }
}

impl FromStr for Line {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<x1>\d+),(?P<y1>\d+)\W->\W(?P<x2>\d+),(?P<y2>\d+)$").unwrap();
        }

        let captures = RE.captures(s).ok_or(AdventError::InvalidRegex)?;

        let x1 = parse_from_named_captures_or(&captures, "x1", AdventError::InvalidRegex)?;
        let y1 = parse_from_named_captures_or(&captures, "y1", AdventError::InvalidRegex)?;

        let x2 = parse_from_named_captures_or(&captures, "x2", AdventError::InvalidRegex)?;
        let y2 = parse_from_named_captures_or(&captures, "y2", AdventError::InvalidRegex)?;

        Ok(Line {
            start: Coord::new(x1, y1),
            end: Coord::new(x2, y2),
        })
    }
}
