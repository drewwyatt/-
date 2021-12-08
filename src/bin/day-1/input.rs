use std::fs;

pub fn read() -> std::io::Result<Vec<i64>> {
  let path = "./src/bin/day-1/INPUT.txt";
  let contents = fs::read_to_string(path)?;

  let values = contents
    .lines()
    .map(|l| l.parse().unwrap())
    .collect::<Vec<i64>>();

  Ok(values)
}
