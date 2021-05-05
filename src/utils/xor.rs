pub fn xor_byte(a: u8, b: u8) -> u8 {
  a ^ b
}

pub fn xor_equal_length(a: &Vec<u8>, b: &Vec<u8>) -> Result<Vec<u8>, String> {
  if a.len() != b.len() {
    Err(String::from("Not the same length"))
  } else {
    Ok(a.iter()
        .zip(b.iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect()
        )
  }
}
