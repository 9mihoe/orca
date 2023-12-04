use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  let path = "/Users/gowoonkim/Desktop/orca/src/day1.txt";
  let part2 = day1_part2(path.to_string());
  println!("{}", part2);

  // let ex = get_calibration_value2("eighthree".to_string());
  // println!("{}", ex);
}

fn day1_part1(path: String) -> u32 {
  let mut sum = 0;
  if let Ok(lines) = read_lines(path) {
    for line in lines {
      if let Ok(txt) = line {
        sum += get_calibration_value(txt);
      }
    }
  }
  return sum;
}

fn day1_part2(path: String) -> u32 {
  let mut sum = 0;
  if let Ok(lines) = read_lines(path) {
    for line in lines {
      if let Ok(txt) = line {
        sum += get_calibration_value2(txt);
      }
    }
  }
  return sum;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// Copied from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn get_calibration_value2(s: String) -> u32 {
  let mut it = 0;
  let mut cat = String::new();
  while it < s.len() {
    if let Some(_digit) = s.chars().nth(it).unwrap().to_digit(10) {
      cat.push(s.chars().nth(it).unwrap());
    }
    else if let Some(three_char_digit) = char_match(&s, 3, it) {
      cat.push(three_char_digit);
    }
    else if let Some(four_char_digit) = char_match(&s, 4, it) {
      cat.push(four_char_digit);
    }
    else if let Some(five_char_digit) = char_match(&s, 5, it) {
      cat.push(five_char_digit);
    }
    it += 1;
    // println!("{} {}", cat, it);
  }
  return str_first_last_int(cat);
}

fn vec_first_last_int(chars: Vec<char>) -> u32 {
  return format!("{}{}", chars.first().unwrap(), chars.last().unwrap()).parse::<u32>().unwrap();
}

fn str_first_last_int(chars: String) -> u32 {
  return format!("{}{}", chars.chars().nth(0).unwrap(), chars.chars().last().unwrap()).parse::<u32>().unwrap();
}

fn get_calibration_value(s: String) -> u32 {
  let chars : Vec<char> = s.chars().filter(|&i| i.is_ascii_digit()).collect();
  return vec_first_last_int(chars);
}

fn str_to_digit(s: &str) -> Option<char> {
  match s {
    "one" => Some('1'),
    "two" => Some('2'),
    "three" => Some('3'),
    "four" => Some('4'),
    "five" => Some('5'),
    "six" => Some('6'),
    "seven" => Some('7'),
    "eight" => Some('8'),
    "nine" => Some('9'),
    _ => None
  }
}

fn char_match(s: &str, n: usize, it: usize) -> Option<char> {
  if s.len() < n || it > s.len()-n {
    return None;
  }
  str_to_digit(&s[it..it+n])
}
