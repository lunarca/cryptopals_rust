use crate::utils::fileio::load_ciphertexts_lines;
use crate::utils::xor::find_fixed_byte_xor;

pub fn run() -> bool {

  println!("Cryptopals 1-4");

  println!("Loading challenge from file...");
  
  let filename = "./assets/1_4_challenge.txt";
  let ciphertexts = load_ciphertexts_lines(filename);
  
  println!("Done. Starting crack...");

  match find_fixed_byte_xor(ciphertexts) {
    Some((best_key, best_plaintext, best_score)) => {
      println!("Good crack. Found best key of {} with score {}", best_key, best_score);
      println!("{}", String::from_utf8(best_plaintext).unwrap());
      return true;
    },
    None => {
      println!("No good crack.");
      return false
    }
  };


}