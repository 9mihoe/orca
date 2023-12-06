use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq)]
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
    let mut line_symbols = vec![];
    let mut line_numbers = vec![];
    let mut y = 0;
    for line in lines {
      if let Ok(txt) = line {
        line_symbols.push(get_symbols(&txt, y));
        line_numbers.push(get_numbers(&txt, y));
      }
      y += 1;
    }
    let symbols = line_symbols.into_iter().flatten().collect::<Vec<Point>>();
    let numbers = line_numbers.into_iter().flatten().collect::<Vec<Number>>();
    let sum = get_sum(symbols, numbers);
    return Some(sum);
  }
  return None;
}

fn get_symbols(line: &str, y: usize) -> Vec<Point> {
  let sym_it =
    line
    .chars()
    .enumerate()
    .filter_map(
      |(k, v)|
      if v!='.' && !v.is_ascii_digit() {
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

fn get_start_stop(p: usize, len: usize) -> (usize, usize) {
  let start : usize = if p==0 { 0 } else { p-1 };
  let stop : usize = if p==0 { start+len+1 } else { start+len+2 };
  (start, stop)
}

fn get_window(number: &Number) -> Vec<Point> {
  let mut points = vec![];
  let (xstart, xstop) = get_start_stop(number.start.x, number.digits.len());
  let (ystart, ystop) = get_start_stop(number.start.y, 1);
  // println!("xstart: {}, xstop: {}, ystart: {}, ystop: {}", xstart, xstop, ystart, ystop);
  for x in xstart..xstop {
    for y in ystart..ystop {
      points.push(Point{x:x, y:y});
    }
  }
  return points;
}

fn get_sum(symbols: Vec<Point>, numbers: Vec<Number>) -> u32 {
  // println!("symbols: {:?}", symbols);
  let mut sum : u32 = 0;
  for number in numbers {
    let window = get_window(&number);
    // println!("number: {:?} window: {:?}", number, window);
    if window.iter().any(|p| symbols.contains(p)) {
      let digits = number.digits.parse::<u32>().unwrap();
      // println!("{}", digits);
      sum += number.digits.parse::<u32>().unwrap();
    }
  }
  return sum;
}