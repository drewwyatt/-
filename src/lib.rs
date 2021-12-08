use std::fs;
use std::str::FromStr;

pub fn read_input_for_day<T>(day: usize) -> std::io::Result<Vec<T>>
where
  T: FromStr,
  T::Err: std::fmt::Debug,
{
  let path = format!("./src/bin/day-{}/INPUT.txt", day);
  let contents = fs::read_to_string(path)?;

  let values = contents
    .lines()
    .map(|l| l.parse().unwrap())
    .collect::<Vec<T>>();

  Ok(values)
}
