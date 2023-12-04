use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;

#[derive(Debug)]
struct RGB {
  r: u32,
  g: u32,
  b: u32
}

#[derive(Debug)]
struct Game {
  id : u32,
  cubes: Vec<RGB>
}

fn main() {
  let path = "/Users/gowoonkim/Desktop/orca/src/day2.txt";
  let games = get_games(path.to_string());
  // let valid_id_sum = valid_id_sum(games, 12, 13, 14);
  let powers = power_all_cubes(games);
  println!("{:?}", powers);
}

fn valid_id_sum(games: Vec<Game>, mred: u32, mgreen: u32, mblue: u32) -> u32 {
  let mut sum = 0;
  for game in games {
    if let Some(id) = valid_game_id(game, mred, mgreen, mblue) {
      sum += id;
    }
  }
  return sum;
}

fn min_cubes(game: Game) -> RGB {
  let mut reds = vec![];
  let mut greens = vec![];
  let mut blues = vec![];
  for cube in game.cubes {
    reds.push(cube.r);
    greens.push(cube.g);
    blues.push(cube.b);
  }
  RGB{
    r: *reds.iter().max().unwrap(),
    g: *greens.iter().max().unwrap(),
    b: *blues.iter().max().unwrap()
  }
}

fn power_cubes(game: Game) -> u32 {
  let rgb = min_cubes(game);
  return rgb.r * rgb.g * rgb.b;
}

fn power_all_cubes(games: Vec<Game>) -> u32 {
  let mut sum = 0;
  for game in games {
    sum += power_cubes(game);
  }
  return sum;
}

fn valid_game_id(game: Game, mred: u32, mgreen: u32, mblue: u32) -> Option<u32> {
  let mut reds : u32 = 0;
  let mut greens : u32 = 0;
  let mut blues : u32 = 0;
  for cube in game.cubes {
    reds = cmp::max(cube.r, reds);
    greens = cmp::max(cube.g, greens);
    blues = cmp::max(cube.b, blues);
  }
  if reds<=mred && greens<=mgreen && blues<=mblue {
    return Some(game.id);
  }
  return None;
}

fn parse(line: String) -> Game {
  let l : Vec<&str> = line.split(": ").collect();
  let id : u32 = l.first().unwrap().split_whitespace().nth(1).unwrap().parse::<u32>().unwrap();
  let mut games = vec![];
  for subgame in l.last().unwrap().split("; ") {
    games.push(parse_cubes(subgame));
  }
  Game {
    id : id,
    cubes : games
  }
}

fn parse_cubes(cubes: &str) -> RGB {
  let mut res = RGB {r:0, g:0, b:0};
  for cube in cubes.split(", ") {
    let count : u32 = cube.split_whitespace().nth(0).unwrap().parse::<u32>().unwrap();
    if cube.contains("red") {
      res.r = count;
    }
    if cube.contains("green") {
      res.g = count;
    }
    if cube.contains("blue") {
      res.b = count;
    }
  }
  return res;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// Copied from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn get_games(path: String) -> Vec<Game> {
  let mut games = vec![];
  if let Ok(lines) = read_lines(path) {
    for line in lines {
      if let Ok(txt) = line {
        games.push(parse(txt));
      }
    }
  }
  return games;
}