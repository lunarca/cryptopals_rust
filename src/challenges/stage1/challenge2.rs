use hex;
use crate::utils::xor;

pub fn run() -> bool {
  let input_1_str = String::from("1c0111001f010100061a024b53535009181c");
  let input_2_str = String::from("686974207468652062756c6c277320657965");

  let output_challenge_str = String::from("746865206b696420646f6e277420706c6179");

  println!("Challenge: {} ^ {} == {}", input_1_str, input_2_str, output_challenge_str);

  // Not doing error handling here because these must decode.
  let input_1_bytes = hex::decode(&input_1_str).unwrap();
  let input_2_bytes = hex::decode(&input_2_str).unwrap();
  let output_challenge_bytes = hex::decode(&output_challenge_str).unwrap();

  let result = xor::xor_equal_length(&input_1_bytes, &input_2_bytes);

  match result {
    Ok(value) => {
      let result_str = hex::encode(&value);
      println!("XOR'd value: {}", result_str);
      let success = value == output_challenge_bytes;
      println!("Success: {}", success);
      success
    },

    Err(error) => { println!("Error: {}", error); false }
  }
}