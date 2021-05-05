use base64;

pub fn hex_to_b64(hex_string: String) -> Result<String, hex::FromHexError> {
  hex::decode(hex_string)
    .map(|value| base64::encode(value))
}