
use crate::utils::hex;

pub fn run() -> bool {
  let hex_string = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
  let challenge_str = String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");

  println!("Test: b64(hex_decode({})) === {}", hex_string, challenge_str);

  let decoded = hex::hex_to_b64(hex_string);

  match decoded {
    Ok(value) => {
      println!("Decode success: {}", value);
      let value_matches = value == challenge_str;
      println!("Success: {}", value_matches);
      value_matches
    },

    Err(_) => { 
      println!("Error: Could not parse input");
      false
    }
  }
}