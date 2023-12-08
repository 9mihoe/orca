use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::collections::HashMap;

fn get_points(s: String) -> u32 {
  let wins : HashSet<u32> =
    s
    .split(": ")
    .nth(1)
    .unwrap()
    .split(" | ")
    .nth(0)
    .unwrap()
    .split_whitespace()
    .map(|x| x.parse::<u32>().unwrap())
    .collect();

  let cards : HashSet<u32> = 
    s
    .split(": ")
    .nth(1)
    .unwrap()
    .split(" | ")
    .nth(1)
    .unwrap()
    .split_whitespace()
    .map(|x| x.parse::<u32>().unwrap())
    .collect();

  // let base : u32 = 2;
  let mine : u32 = cards.intersection(&wins).count() as u32;
  // return if mine==0 { 0 } else { base.pow(mine-1) };
  return mine;
}

fn main() {
  let path = "/Users/gowoonkim/Desktop/orca/src/day4.txt";
  let res = solve(path.to_string());
  println!("{}", res);

  // let line = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
  // let points = get_points(line.to_string());
  // println!("{}", points);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn solve(path: String) -> u32 {
  let mut tally = HashMap::new();
  for ctr in 1..202 {
    tally.insert(ctr, 1);
  }
  if let Ok(lines) = read_lines(path) {
    for (i, line) in lines.enumerate() {
      if let Ok(txt) = line {
        let card = i+1;
        let points = get_points(txt);
        println!("card: {}, points: {}", card, points);
        for _ in 0..tally[&card] {
          if points > 0 {
            for ctr in card+1..(card+(points as usize)+1) {
              tally.insert(ctr, tally[&ctr]+1);
            }
          }
        }
      }
    }
  }
  // println!("{:?}", tally);
  return tally.values().sum()
}