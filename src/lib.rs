extern crate num;

use num::{bigint::{BigUint, ToBigUint}, traits::Pow};

// Valid chars for use in base58 encode
const CHARS: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

// Converts string byte to its value
const BYTE_MAP: [i8; 256] = [
    -1,-1,-1,-1,-1,-1,-1,-1, -1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, -1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, -1,-1,-1,-1,-1,-1,-1,-1,
    -1, 0, 1, 2, 3, 4, 5, 6,  7, 8,-1,-1,-1,-1,-1,-1,
    -1, 9,10,11,12,13,14,15, 16,-1,17,18,19,20,21,-1,
    22,23,24,25,26,27,28,29, 30,31,32,-1,-1,-1,-1,-1,
    -1,33,34,35,36,37,38,39, 40,41,42,43,-1,44,45,46,
    47,48,49,50,51,52,53,54, 55,56,57,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, -1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, -1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, -1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, -1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, -1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, -1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, -1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, -1,-1,-1,-1,-1,-1,-1,-1,
];

/// Takes any hex string and returns the base58 encoded string
///
/// # Arguments
///
/// * `input` - hex string to be encoded
///
/// # Example
/// ```
/// encode("f54a5851e9372b87810a8e60cdd2e7cfd80b6e31c7f18fe8");
/// // |-> "PMycacnJaSqwwJqjawXBErnLsZ7RkXUAs"
/// ```
pub fn encode(input: &str) -> String {
    let mut number =
        BigUint::parse_bytes(input.as_bytes(), 16).expect("Input not valid hex string");

    let mut output = String::new();

    let zero = ToBigUint::to_biguint(&0).unwrap();
    let fifty_eight = ToBigUint::to_biguint(&58).unwrap();

    while number > zero {
        let bit = &number % &fifty_eight;
        let bit: usize = bit.to_str_radix(10).parse().unwrap();

        number /= &fifty_eight;

        output += &CHARS[bit..bit + 1];
    }

    let reversed: String = output.chars().rev().collect();

    reversed
}

/// Takes any base58 string and returns decoded hex string
///
/// # Arguments
///
/// `input` - base58 string to decode
///
/// # Example
/// ```
/// decode("PMycacnJaSqwwJqjawXBErnLsZ7RkXUAs");
/// // |-> "f54a5851e9372b87810a8e60cdd2e7cfd80b6e31c7f18fe8"
/// ```
pub fn decode(input: &str) -> String {
    let mut number = ToBigUint::to_biguint(&0).unwrap();

    for (index, byte) in input.bytes().rev().enumerate() {
        let value = ToBigUint::to_biguint(&BYTE_MAP[byte as usize]).unwrap();
        number += value * (ToBigUint::to_biguint(&58).unwrap().pow(&index));
    }

    number.to_str_radix(16)
}

#[cfg(test)]
mod tests {
    #[test]
    fn encode_test() {
        assert_eq!(super::encode("f54a5851e9372b87810a8e60cdd2e7cfd80b6e31c7f18fe8"), "PMycacnJaSqwwJqjawXBErnLsZ7RkXUAs");
    }

    #[test]
    fn decode_test() {
        assert_eq!(super::decode("PMycacnJaSqwwJqjawXBErnLsZ7RkXUAs"), "f54a5851e9372b87810a8e60cdd2e7cfd80b6e31c7f18fe8");
    }
}
