use crate::utils::scoring;
use rayon::prelude::*;

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


pub fn fixed_byte_xor(key: u8, target: &Vec<u8>) -> Vec<u8> {
  let expanded_key = expand_single_byte_key(key, target.len());
  let result = xor_equal_length(&expanded_key, target);
  return result.unwrap()
}


fn expand_single_byte_key(key: u8, len: usize) -> Vec<u8> {
  return vec![key; len];
}

pub fn crack_single_byte_xor(ciphertext: &Vec<u8>) -> Option<(u8, Vec<u8>, i32)> {
  let possible_keys: Vec<u8> = (0..255).collect();

  let mut results: Vec<(u8, Vec<u8>, i32)> = possible_keys.par_iter()
    .map(|&key| {
      let plaintext = fixed_byte_xor(key, ciphertext);
      let score = scoring::score_english_plaintext(&plaintext);
      return (key, plaintext, score)
    }).collect();

  results.sort_by(|a, b| a.2.cmp(&b.2));
  // return match results.pop() {
  //   Some((key, plaintext, score)) => Some((key, plaintext, score)),
  //   None => None,
  // }

  return results.pop()
}


pub fn find_fixed_byte_xor(possible_ciphertexts: Vec<Vec<u8>>) -> Option<(u8, Vec<u8>, i32)> {
  let mut cracked_results: Vec<(u8, Vec<u8>, i32)> = possible_ciphertexts.iter()
    .filter_map(crack_single_byte_xor)
    .collect();

  cracked_results.sort_by(|a, b| a.2.cmp(&b.2));
  
  return cracked_results.pop()
}