use crate::chal1::hex_to_bytes;
use std::{collections::HashMap, fs};

/// This function detects AES in ECB mode. It reads the file set1/data/chal8/8.txt line by line.
/// It then checks if the line has duplicate chunks of bytes. If it does, it prints the line.
/// Otherwise, it does nothing.
///
/// The line with duplicate chunks of bytes is the line that is most likely to be encrypted with
/// AES in ECB mode. This is because AES in ECB mode encrypts the plaintext in blocks of 16 bytes.
/// The likelihood of having duplicate chunks of bytes in a line encrypted with AES in ECB mode is
/// high.
pub fn detect_aes_in_ecb_mode() {
    // read the file line by line.
    let cipher_reader = fs::read_to_string("set1/data/chal8/8.txt").unwrap();
    cipher_reader.lines().for_each(|cipher| {
        let cipher_bytes = hex_to_bytes(cipher).unwrap();
        if duplicate_checker(&cipher_bytes, 16) {
            println!("Cipher: {}", cipher);
        }
    });
}

/// This function checks if a cipher has duplicate chunks of bytes. If it does, it returns true.
/// Otherwise, it returns false.
fn duplicate_checker(cipher: &[u8], chunks_len: usize) -> bool {
    let mut has_duplicate = false;

    let mut map = HashMap::new();
    cipher
        .chunks_exact(chunks_len)
        .enumerate()
        .for_each(|(_, chunk)| {
            if map.contains_key(chunk) {
                has_duplicate = true;
            } else {
                map.insert(chunk, 1);
            }
        });

    has_duplicate
}
