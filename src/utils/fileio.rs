use hex;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn load_ciphertexts_lines(filename: &str) -> Vec<Vec<u8>> {
  return read_file_lines(Path::new(filename).as_ref())
    .filter_map(decode_line)
    .collect();
}

fn decode_line(line: Result<String, std::io::Error>) -> Option<Vec<u8>> {
  return match line {
    Ok(hex_string) => {
      match hex::decode(hex_string) {
        Ok(value) => Some(value),
        Err(_) => None,
      }
    },
    Err(_) => None
  }
}

pub fn read_file_lines(filename: &Path) -> io::Lines<io::BufReader<File>> {
  let display = filename.display();

  let file = match File::open(filename) {
    Err(why) => panic!("Could not open file: {}, {}", display, why),
    Ok(file) => file,
  };
  return io::BufReader::new(file).lines()
}