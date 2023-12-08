use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
  x: usize,
  y: usize
}

#[derive(Debug)]
struct Number {
  digits: String,
  start: Point
}

impl Number {
  fn append(&mut self, c: char) {
    self.digits.push(c);
  }
}

fn main() {
  let path = "/Users/gowoonkim/Desktop/orca/src/day3.txt";
  let res = solve(path.to_string());
  println!("{}", res.unwrap());

  // let ex = "905/.............417.20.....*...............#..................103.............795.....829...611...................653.........%.......882..";
  // get_numbers(ex, 21);

  // let num = Number{digits: "114".to_string(), start: Point{x: 5, y:0}};
  // let ex = get_window(&num);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn solve(path: String) -> Option<u32> {
  if let Ok(lines) = read_lines(path) {
    let mut line_stars = vec![];
    let mut line_numbers = vec![];
    let mut y = 0;
    for line in lines {
      if let Ok(txt) = line {
        line_stars.push(get_stars(&txt, y));
        line_numbers.push(get_numbers(&txt, y));
      }
      y += 1;
    }
    let points = line_stars.into_iter().flatten().collect::<Vec<Point>>();
    let numbers = line_numbers.into_iter().flatten().collect::<Vec<Number>>();
    let mut sum = 0;
    for point in points {
      // println!("point: {:?}", point);
      if let Some(mult) = get_adjacent_multiples(point, &numbers) {
        sum += mult;
      }
    }
    return Some(sum);
  }
  return None;
}

fn get_stars(line: &str, y: usize) -> Vec<Point> {
  let sym_it =
    line
    .chars()
    .enumerate()
    .filter_map(
      |(k, v)|
      if v=='*' {
        Some(k)
      } else {
        None
      }
    );

  let mut sym_pos = vec![];
  for x in sym_it {
    sym_pos.push(Point{x: x, y: y});
  }
  return sym_pos;
}

fn get_numbers(line: &str, y: usize) -> Vec<Number> {
  let digit_pos_it =
    line
    .chars()
    .enumerate()
    .filter_map(
      |(k, v)| 
      if v.is_ascii_digit() {
        Some((k, v))
      } else {
        None
      }
    );
  
  let mut prev : i32 = -1;
  let mut numbers = vec![];
  // Ugly ass.
  for (x, digit) in digit_pos_it {
    if prev==-1 || (x as i32) != prev+1 {
      numbers.push(Number{digits: digit.to_string(), start: Point{x: x, y: y}});
    } else {
      numbers.last_mut().unwrap().append(digit);
    }
    prev = x as i32;
  }
  return numbers;
}

fn get_start_stop(p: usize) -> (usize, usize) {
  let start : usize = if p==0 { 0 } else { p-1 };
  let stop : usize = if p==0 { start+2 } else { start+3 };
  (start, stop)
}

fn get_window(point: &Point) -> HashSet<Point> {
  let mut points = HashSet::new();
  let (xstart, xstop) = get_start_stop(point.x);
  let (ystart, ystop) = get_start_stop(point.y);
  for x in xstart..xstop {
    for y in ystart..ystop {
      points.insert(Point{x:x, y:y});
    }
  }
  return points;
}

fn is_adjacent(star: &Point, number: &Number) -> Option<u32> {
  let window = get_window(&star);
  let mut digits = HashSet::new();
  for i in 0..number.digits.len() {
    digits.insert(Point{x: number.start.x+i, y: number.start.y});
  }
  // println!("star: {:?} number: {:?} window: {:?} digits: {:?}", star, number.digits, window, digits);
  if window.is_disjoint(&digits) {
    return None;
  }
  // println!("HELLO");
  return Some(number.digits.parse::<u32>().unwrap());
}

fn get_adjacent_multiples(star: Point, numbers: &Vec<Number>) -> Option<u32> {
  let mut adj_nums = vec![];
  for number in numbers {
    if let Some(adj_num) = is_adjacent(&star, number) {
      adj_nums.push(adj_num);
      // println!("adj_num: {:?}", adj_num);
      if adj_nums.len()==2 {
        return Some(adj_nums[0] * adj_nums[1]);
      }
      if adj_nums.len()>2 {
        return None;
      }
    }
  }
  return None;
}