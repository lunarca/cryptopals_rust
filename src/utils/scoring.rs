use im::HashMap;

pub fn score_english_plaintext(plaintext: &Vec<u8>) -> i32 {
  let english_frequency: Vec<u8> = vec![
    " ", "e", "t", "o", "a", "h", "n", "s", "r", "i", "l", "d", "\n", "u", "m", "y", ",",
    ".", "w", "f", "c", "g", "I", "b", "p", "A", "E", "T", "S", "v", "O", "'", "k", "R",
    "N", "L", "C", "H", ";", "W", "M", "D", "B", "U", "P", "F", "G", "?", "Y", "!", "-",
    "K", "x", "V", "j", "q", "[", "]", "J", ":", "Q", "z", "9", "1", ")", "(", "X", "Z", "\"",
    "<", ">", "2", "3", "0", "4", "5", "_", "*", "6", "7", "8", "|", "&", "@", "/", "}", "#",
    "=", "%", "~", "`"
  ].iter().map(|charstring| charstring.bytes().next().unwrap()).collect();

  let plaintext_frequencies = calculate_frequencies(plaintext);
  let mut plaintext_freq_array: Vec<_> = plaintext_frequencies.iter().collect();
  plaintext_freq_array.sort_by(|a, b| a.1.cmp(b.1));
  let ordered_plaintext_array = plaintext_freq_array.iter().map(|a| a.0);

  return ordered_plaintext_array.enumerate().fold(0, |acc, (index, &val)| {
    return acc + calculate_distance(val, &english_frequency, index)
  });
}

fn calculate_distance(chr: u8, base_frequency_order: &Vec<u8>, index: usize) -> i32 {
  match base_frequency_order.iter().position(|&value| value == chr) {
    Some(internal_index) => score_distance(index, internal_index),
    None => 0,
  }
}


fn score_distance(ref_index: usize, index: usize) -> i32 {
  if ref_index > index {
    return 30 - (ref_index - index) as i32;
  } else {
    return 30 - (index - ref_index) as i32;
  }
}

pub fn calculate_frequencies(plaintext: &Vec<u8>) -> HashMap<u8, i32> {
  let results: HashMap<u8, i32> = HashMap::new();

  let frequencies = plaintext.iter().fold(results, |acc, &ch| {
    return acc.update_with(ch, 1, |old, new| old + new);
  });

  return frequencies.clone();
}
