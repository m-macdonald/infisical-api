use base64::{engine::general_purpose, Engine as _};

pub fn encode(str: &[u8]) -> String {
    general_purpose::STANDARD.encode(str)
}

pub fn decode(vec: &str) -> Vec<u8> {
    general_purpose::STANDARD.decode(vec).unwrap()
}
