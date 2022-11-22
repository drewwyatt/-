use advent::parse_from_named_captures_or;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

#[derive(Clone, Copy, Debug)]
pub enum AdventError {
    InvalidRegex,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Coord { x: x, y: y }
    }
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

    pub fn segments(&self) -> Vec<Coord> {
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
