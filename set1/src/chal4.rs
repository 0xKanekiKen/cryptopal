use crate::{chal1::hex_to_bytes, chal3::*};
use std::fs;

pub fn detect_single_char_xor() {
    // read the file line by line.
    let cipher_reader = fs::read_to_string("set1/4.txt").unwrap();

    // Find the line with the highest score. The line with the highest score is
    // the line that is most likely to be encrypted with a single character.
    // Once the line is found, decrypt it using the single character with which
    // it was encrypted.
    let mut max_score = 0.0;
    let mut xor_with = 0;
    let mut encrypted_text = String::new();
    cipher_reader.lines().for_each(|cipher| {
        // Get the scores for each byte in the cipher.
        let cipher_score = get_scores(cipher);

        // Find the index of the highest score and the highest score if any.
        cipher_score.iter().enumerate().for_each(|(i, score)| {
            if *score > max_score {
                max_score = *score;
                xor_with = i;
                encrypted_text = cipher.to_string();
            }
        });
    });

    println!("Key: {}", xor_with as u8);
    println!("Score: {}", max_score);
    println!("Encrypted text: {}", encrypted_text);

    // Decrypt the cipher.
    let decrypted_bytes: Vec<u8> = hex_to_bytes(&encrypted_text)
        .unwrap()
        .iter()
        .map(|x| x ^ xor_with as u8)
        .collect();

    println!(
        "Decrypted: {:?}",
        String::from_utf8(decrypted_bytes.clone()).unwrap()
    );

    // save the decrypted text to a file.
    fs::write("set1/4_decrypted.txt", decrypted_bytes).unwrap();
}
