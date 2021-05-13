use crate::utils::xor;
use hex;

pub fn run() -> bool {
  let challenge_str = hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();

  return match xor::crack_single_byte_xor(&challenge_str) {
    Some((key, plaintext, score)) => {
      println!("Cracked it! Key was {} with score {}. Plaintext follows", key, score);
      let str_plaintext = String::from_utf8(plaintext).unwrap();
      println!("{}", str_plaintext);
      true
    },
    None => {
      println!("Couldn't crack it man");
      false
    }
  }

}