use advent::read_input_for_day;
use std::io;

struct Position {
    lines: Vec<u32>,
}

impl Position {
    pub fn new(value: &char) -> Position {
        Position {
            lines: vec![value.to_digit(2).unwrap()],
        }
    }

    pub fn push(&mut self, bit: &char) {
        self.lines.push(bit.to_digit(2).unwrap());
    }

    pub fn get_gamma(&self) -> &str {
        if self.zeroes_count() > self.ones_count() {
            "0"
        } else {
            "1"
        }
    }

    pub fn get_epsilon(&self) -> &str {
        if self.zeroes_count() > self.ones_count() {
            "1"
        } else {
            "0"
        }
    }

    fn zeroes_count(&self) -> usize {
        self.lines.iter().filter(|line| *line == &0).count()
    }

    fn ones_count(&self) -> usize {
        self.lines.iter().filter(|line| *line == &1).count()
    }
}

struct Diagnostic {
    positions: Vec<Position>,
}

impl Diagnostic {
    pub fn new() -> Diagnostic {
        Diagnostic { positions: vec![] }
    }

    pub fn get_gamma(&self) -> String {
        let mut gamma = "".to_owned();
        for position in self.positions.iter() {
            gamma.push_str(position.get_gamma());
        }

        gamma
    }

    pub fn get_epsilon(&self) -> String {
        let mut epsilon = "".to_owned();
        for position in self.positions.iter() {
            epsilon.push_str(position.get_epsilon());
        }

        epsilon
    }

    pub fn push(&mut self, bit_index: usize, value: &char) {
        match self.positions.get_mut(bit_index) {
            Some(position) => position.push(value),
            None => {
                self.positions.push(Position::new(value));
            }
        }
    }
}

fn part_one(input: &Vec<String>) -> (i64, i64) {
    let mut diagnostic = Diagnostic::new();
    for line in input {
        for (bit_index, value) in line.chars().enumerate() {
            diagnostic.push(bit_index, &value);
        }
    }

    (
        i64::from_str_radix(&diagnostic.get_gamma(), 2).unwrap(),
        i64::from_str_radix(&diagnostic.get_epsilon(), 2).unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ]
    }

    #[test]
    fn part_one_sample_input() {
        assert_eq!(part_one(&get_input()), (22, 9))
    }
}

fn main() -> io::Result<()> {
    let input = read_input_for_day(3)?;

    let (gamma, epsilon) = part_one(&input);
    println!(
        "[Day 3, Part 1] Gamma: {}, Epsilon: {}, Power Consumption: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    Ok(())
}
