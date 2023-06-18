use set1::chal1::hex_to_bytes;
use std::collections::HashMap;

/// This function returns a matrix of the frequency of each character in the
/// English language. The matrix is indexed by the XOR of two characters.
/// For example, the value at index [0][0] is the frequency of the character
/// 'a' XOR 'a', which is 0. The value at index [0][1] is the frequency of the
/// character 'a' XOR 'b', which is 0.01492. The value at index [1][0] is the
/// frequency of the character 'b' XOR 'a', which is 0.01492. The value at
/// index [1][1] is the frequency of the character 'b' XOR 'b', which is 0.
///
/// The matrix is used to score the likelihood that a given byte is the key
/// used to encrypt a message. The higher the score, the more likely that the
/// byte is the key.
fn get_char_freq_matrix() -> [[f64; 256]; 256] {
    // The frequency of each character in the English language.
    let mut char_freq = HashMap::new();

    char_freq.insert(b'a', 0.08167);
    char_freq.insert(b'b', 0.01492);
    char_freq.insert(b'c', 0.02782);
    char_freq.insert(b'd', 0.04253);
    char_freq.insert(b'e', 0.12702);
    char_freq.insert(b'f', 0.02228);
    char_freq.insert(b'g', 0.02015);
    char_freq.insert(b'h', 0.06094);
    char_freq.insert(b'i', 0.06966);
    char_freq.insert(b'j', 0.00153);
    char_freq.insert(b'k', 0.00772);
    char_freq.insert(b'l', 0.04025);
    char_freq.insert(b'm', 0.02406);
    char_freq.insert(b'n', 0.06749);
    char_freq.insert(b'o', 0.07507);
    char_freq.insert(b'p', 0.01929);
    char_freq.insert(b'q', 0.00095);
    char_freq.insert(b'r', 0.05987);
    char_freq.insert(b's', 0.06327);
    char_freq.insert(b't', 0.09056);
    char_freq.insert(b'u', 0.02758);
    char_freq.insert(b'v', 0.00978);
    char_freq.insert(b'w', 0.02360);
    char_freq.insert(b'x', 0.00150);
    char_freq.insert(b'y', 0.01974);
    char_freq.insert(b'z', 0.00074);
    char_freq.insert(b' ', 0.13000);

    let mut char_freq_matrix = [[0.0; 256]; 256];

    // Iterate over the frequencies and populate the matrix.
    char_freq.iter().enumerate().for_each(|(_, (k, v))| {
        char_freq_matrix
            .iter_mut()
            .enumerate()
            .for_each(|(j, row)| {
                row[(k ^ j as u8) as usize] = *v;
            });
    });

    char_freq_matrix
}

/// This function returns an array of the frequency of each character in the
/// cipher. The array is indexed by the byte value of the character.
fn compute_freq_in_cipher(cipher: &str) -> [f64; 256] {
    // Convert the cipher from a hex string to a byte array.
    let cipher_in_bytes = hex_to_bytes(cipher).unwrap();

    // An array to hold the frequency of each character in the cipher.
    let mut char_counter = [0.0; 256];
    let size = cipher_in_bytes.len() as f64;

    // iterate over the bytes and count the number of times each byte appears.
    for byte in cipher_in_bytes {
        char_counter[byte as usize] += 1.0 / size;
    }

    char_counter
}

/// This function returns the dot product of two arrays.
fn dot_product(a: &[f64; 256], b: &[f64; 256]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// This function returns an array of scores for each byte in the cipher.
fn get_scores(cipher: &str) -> [f64; 256] {
    let char_freq_matrix = get_char_freq_matrix();
    let cipher_freq = compute_freq_in_cipher(cipher);
    let mut scores = [0.0; 256];

    for i in 0..256 {
        scores[i] = dot_product(&char_freq_matrix[i], &cipher_freq);
    }

    scores
}

fn main() {
    let cipher = "1b37373331363f78151b7f2b783431333d78397828372d363c\
                  78373e783a393b3736";
    let cipher_bytes = hex_to_bytes(cipher).unwrap();
    let scores = get_scores(cipher);

    // Find the index of the highest score and the highest score.
    let mut max_score = 0.0;
    let mut max_score_index = 0;
    scores.iter().enumerate().for_each(|(i, score)| {
        if *score > max_score {
            max_score = *score;
            max_score_index = i;
        }
    });

    println!("Key: {}", max_score_index as u8);

    // Decrypt the cipher.
    let decrypted_bytes: Vec<u8> = cipher_bytes
        .iter()
        .map(|x| x ^ max_score_index as u8)
        .collect();

    println!(
        "Decrypted: {:?}",
        String::from_utf8(decrypted_bytes).unwrap()
    );
}
