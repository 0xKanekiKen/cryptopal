use crate::{chal1::hex_to_bytes, chal3::get_scores, chal5::repeating_key_xor};
use std::{collections::HashMap, fs};

fn base64_to_bytes(cipher: &str) -> Result<Vec<u8>, &str> {
    // the length of the cipher should be divisible by 4.
    if cipher.len() % 4 != 0 {
        return Err("Invalid base64 string, it should be divisible by 4");
    }

    let mut cipher_bytes = Vec::new();
    let char_set = HashMap::from([
        ('A', 0b0000_0000),
        ('B', 0b0000_0001),
        ('C', 0b0000_0010),
        ('D', 0b0000_0011),
        ('E', 0b0000_0100),
        ('F', 0b0000_0101),
        ('G', 0b0000_0110),
        ('H', 0b0000_0111),
        ('I', 0b0000_1000),
        ('J', 0b0000_1001),
        ('K', 0b0000_1010),
        ('L', 0b0000_1011),
        ('M', 0b0000_1100),
        ('N', 0b0000_1101),
        ('O', 0b0000_1110),
        ('P', 0b0000_1111),
        ('Q', 0b0001_0000),
        ('R', 0b0001_0001),
        ('S', 0b0001_0010),
        ('T', 0b0001_0011),
        ('U', 0b0001_0100),
        ('V', 0b0001_0101),
        ('W', 0b0001_0110),
        ('X', 0b0001_0111),
        ('Y', 0b0001_1000),
        ('Z', 0b0001_1001),
        ('a', 0b0001_1010),
        ('b', 0b0001_1011),
        ('c', 0b0001_1100),
        ('d', 0b0001_1101),
        ('e', 0b0001_1110),
        ('f', 0b0001_1111),
        ('g', 0b0010_0000),
        ('h', 0b0010_0001),
        ('i', 0b0010_0010),
        ('j', 0b0010_0011),
        ('k', 0b0010_0100),
        ('l', 0b0010_0101),
        ('m', 0b0010_0110),
        ('n', 0b0010_0111),
        ('o', 0b0010_1000),
        ('p', 0b0010_1001),
        ('q', 0b0010_1010),
        ('r', 0b0010_1011),
        ('s', 0b0010_1100),
        ('t', 0b0010_1101),
        ('u', 0b0010_1110),
        ('v', 0b0010_1111),
        ('w', 0b0011_0000),
        ('x', 0b0011_0001),
        ('y', 0b0011_0010),
        ('z', 0b0011_0011),
        ('0', 0b0011_0100),
        ('1', 0b0011_0101),
        ('2', 0b0011_0110),
        ('3', 0b0011_0111),
        ('4', 0b0011_1000),
        ('5', 0b0011_1001),
        ('6', 0b0011_1010),
        ('7', 0b0011_1011),
        ('8', 0b0011_1100),
        ('9', 0b0011_1101),
        ('+', 0b0011_1110),
        ('/', 0b0011_1111),
    ]);

    let size = cipher.len();

    // iterating over chunks of 4 bytes barring the last chunk.
    // the last chunk will be handled separately.
    for i in (0..size - 4).step_by(4) {
        // first octet is the first 6 bits of the first byte and the first 2 bits of the second byte.
        let first_octet = (char_set.get(&cipher.chars().nth(i).unwrap()).unwrap() << 2
            | char_set.get(&cipher.chars().nth(i + 1).unwrap()).unwrap() >> 4)
            as u8;

        // second octet is the last 4 bits of the second byte and the first 4 bits of the third byte.
        let second_octet = char_set.get(&cipher.chars().nth(i + 1).unwrap()).unwrap() << 4
            | char_set.get(&cipher.chars().nth(i + 2).unwrap()).unwrap() >> 2 as u8;

        // third octet is the last 2 bits of the third byte and the first 6 bits of the fourth byte.
        let third_octet = char_set.get(&cipher.chars().nth(i + 2).unwrap()).unwrap() << 6
            | *char_set.get(&cipher.chars().nth(i + 3).unwrap()).unwrap() as u8;

        // pushing the octets to the cipher bytes.
        cipher_bytes.push(first_octet);
        cipher_bytes.push(second_octet);
        cipher_bytes.push(third_octet);
    }

    // processing the last chunk.
    let last_chunk = &cipher[size - 4..];
    // check the occurence of '=' in the last chunk.
    let _ = match last_chunk.matches('=').count() {
        1 => {
            // if there is one `=` in the last chunk, then there would be only two octets in the last chunk.
            let first_octet = (char_set.get(&last_chunk.chars().nth(0).unwrap()).unwrap() << 2
                | char_set.get(&last_chunk.chars().nth(1).unwrap()).unwrap() >> 4)
                as u8;

            let second_octet = char_set.get(&last_chunk.chars().nth(1).unwrap()).unwrap() << 4
                | char_set.get(&last_chunk.chars().nth(2).unwrap()).unwrap() >> 2 as u8;

            cipher_bytes.push(first_octet);
            cipher_bytes.push(second_octet);
        }
        2 => {
            // if there are two `=` in the last chunk, then there will be only one octet in the last chunk.
            let first_octet = (char_set.get(&last_chunk.chars().nth(0).unwrap()).unwrap() << 2
                | char_set.get(&last_chunk.chars().nth(1).unwrap()).unwrap() >> 4)
                as u8;

            cipher_bytes.push(first_octet);
        }
        0 => {
            // first octet is the first 6 bits of the first byte and the first 2 bits of the second byte.
            let first_octet = (char_set.get(&last_chunk.chars().nth(0).unwrap()).unwrap() << 2
                | char_set.get(&last_chunk.chars().nth(1).unwrap()).unwrap() >> 4)
                as u8;

            // second octet is the last 4 bits of the second byte and the first 4 bits of the third byte.
            let second_octet = char_set.get(&last_chunk.chars().nth(1).unwrap()).unwrap() << 4
                | char_set.get(&last_chunk.chars().nth(2).unwrap()).unwrap() >> 2 as u8;

            // third octet is the last 2 bits of the third byte and the first 6 bits of the fourth byte.
            let third_octet = char_set.get(&last_chunk.chars().nth(2).unwrap()).unwrap() << 6
                | *char_set.get(&last_chunk.chars().nth(3).unwrap()).unwrap() as u8;

            // pushing the octets to the cipher bytes.
            cipher_bytes.push(first_octet);
            cipher_bytes.push(second_octet);
            cipher_bytes.push(third_octet);
        }
        _ => {
            return Err("Invalid base64 string");
        }
    };

    Ok(cipher_bytes)
}

fn edit_distance<'a>(s1: &[u8], s2: &[u8]) -> Result<usize, &'a str> {
    // if the strings are of different lengths, then return an error.
    if s1.len() != s2.len() {
        return Err("Strings are of different lengths");
    };

    let mut distance: usize = 0;

    s1.iter()
        .zip(s2.iter())
        .enumerate()
        .for_each(|(_, (c1, c2))| {
            let xor = c1 ^ c2;
            distance += xor.count_ones() as usize;
        });

    Ok(distance)
}

fn keysize_score(cipher: &[u8], max_keysize: usize) -> Result<Vec<f64>, &str> {
    let mut scores: Vec<f64> = Vec::new();
    for size in 1..max_keysize {
        // dividing the cipher into chunks of size `size`.
        let first_chunk = &cipher[0..size];
        let second_chunk = &cipher[size..size * 2];
        let third_chunk = &cipher[size * 2..size * 3];
        let fourth_chunk = &cipher[size * 3..size * 4];

        // calculating the edit distance between the chunks.
        let result_first_second = edit_distance(first_chunk, second_chunk).unwrap() as f64;
        let result_second_third = edit_distance(second_chunk, third_chunk).unwrap() as f64;
        let result_third_fourth = edit_distance(third_chunk, fourth_chunk).unwrap() as f64;
        let result_fourth_first = edit_distance(fourth_chunk, first_chunk).unwrap() as f64;
        let result_first_third = edit_distance(first_chunk, third_chunk).unwrap() as f64;
        let result_second_fourth = edit_distance(second_chunk, fourth_chunk).unwrap() as f64;

        // calculating the average edit distance.
        let result: f64 = (result_first_second
            + result_second_third
            + result_third_fourth
            + result_fourth_first
            + result_first_third
            + result_second_fourth)
            / (6.0 * size as f64);

        // pushing the average edit distance to the scores vector.
        scores.push(result);
    }
    Ok(scores)
}

fn transpose_cipher_into_matrix(cipher: &[u8], keysize: usize) -> Vec<Vec<u8>> {
    let mut matrix = vec![vec![]; keysize];

    // transposing the cipher into a matrix.
    for (i, byte) in cipher.iter().enumerate() {
        matrix[i % keysize].push(*byte);
    }

    matrix
}

pub fn find_key(cipher: &str, max_keysize: usize) -> Vec<Vec<u8>> {
    let mut top_keys = vec![];
    let cipher_bytes = base64_to_bytes(cipher).unwrap();
    let mut score_per_key = keysize_score(&cipher_bytes, max_keysize).unwrap();

    // Returns the index of bottom scores.
    let mut keysize: Vec<usize> = Vec::with_capacity(1);
    for _ in 0..1 {
        let mut min_score = f64::MAX;
        let mut index = 0;
        score_per_key.iter().enumerate().for_each(|(i, score)| {
            if *score < min_score {
                min_score = *score;
                index = i;
            }
        });
        keysize.push(index + 1);
        score_per_key[index] = f64::MAX;
    }

    for size in keysize {
        let transpose_matrix = transpose_cipher_into_matrix(&cipher_bytes, size);

        let mut keys: Vec<u8> = Vec::with_capacity(size);
        for rows in transpose_matrix {
            let scores = get_scores(&rows);

            // Find the index of the highest score and the highest score.
            let mut max_score = 0.0;
            let mut key = 0;
            scores.iter().enumerate().for_each(|(i, score)| {
                if *score > max_score {
                    max_score = *score;
                    key = i;
                }
            });
            keys.push(key as u8);
        }
        top_keys.push(keys)
    }

    top_keys
}

pub fn break_repeating_key_xor() {
    let cipher: String = fs::read_to_string("set1/data/chal6/6.txt")
        .unwrap()
        .parse::<String>()
        .unwrap();

    // replace the newline characters with empty strings.
    let cipher = cipher.replace("\n", "");

    // cipher is in base64 format, so we need to convert it into bytes.
    let cipher_bytes = base64_to_bytes(&cipher).unwrap();

    // find the key. the keys are sorted in descending order of their scores.
    let keys = find_key(&cipher, 40);
    let mut decrypted_bytes: Vec<u8> = Vec::new();

    for key in keys {
        println!("Key: {}", String::from_utf8(key.clone()).unwrap());
        println!("Key length: {}", key.len());

        let hex_decrypted = repeating_key_xor(
            &String::from_utf8(cipher_bytes.clone()).unwrap(),
            &String::from_utf8(key).unwrap(),
        );
        let decrypted_text_bytes = hex_to_bytes(&hex_decrypted).unwrap();

        // printing the plaintext by converting hex string into ascii.
        println!(
            "Decrypted text: {}",
            String::from_utf8(decrypted_text_bytes.clone()).unwrap()
        );

        decrypted_bytes = decrypted_text_bytes;
    }

    // save the decrypted text to a file.
    fs::write("set1/data/chal6/6_decrypted.txt", &decrypted_bytes).unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_base64_to_bytes() {
        //------------------------- no `=` padding in the last chunk -------------------------
        let base64_string = "SGVsbG8gV29ybGQh";
        let expected_bytes = vec![
            0b01001000, 0b01100101, 0b01101100, 0b01101100, 0b01101111, 0b00100000, 0b01010111,
            0b01101111, 0b01110010, 0b01101100, 0b01100100, 0b00100001,
        ];

        let result = base64_to_bytes(base64_string).unwrap();

        assert_eq!(result, expected_bytes);

        //------------------------- one `=` padding in the last chunk -------------------------

        let base64_string = "SGVsbG8gV29ybGQ=";
        let expected_bytes = vec![
            0b01001000, 0b01100101, 0b01101100, 0b01101100, 0b01101111, 0b00100000, 0b01010111,
            0b01101111, 0b01110010, 0b01101100, 0b01100100,
        ];

        let result = base64_to_bytes(base64_string).unwrap();

        assert_eq!(result, expected_bytes);

        //------------------------- two `=` padding in the last chunk -------------------------

        let base64_string = "SGVsbG8gV29ybG==";
        let expected_bytes = vec![
            0b01001000, 0b01100101, 0b01101100, 0b01101100, 0b01101111, 0b00100000, 0b01010111,
            0b01101111, 0b01110010, 0b01101100,
        ];

        let result = base64_to_bytes(base64_string).unwrap();

        assert_eq!(result, expected_bytes);

        //------------------------- invalid base64 string -------------------------

        let base64_string = "SGVsbG8gV29ybG";
        let result = base64_to_bytes(base64_string);

        assert_eq!(
            result,
            Err("Invalid base64 string, it should be divisible by 4")
        );
    }

    #[test]
    fn test_edit_distance() {
        //------------------------- equal length strings -------------------------
        let s1 = "this is a test";
        let s2 = "wokka wokka!!!";
        let expected_distance = 37;
        let result = edit_distance(s1.as_bytes(), s2.as_bytes()).unwrap();

        assert_eq!(result, expected_distance);

        //------------------------- unequal length strings -------------------------
        let s1 = "this is a test";
        let s2 = "wokka wokka!!";
        let result = edit_distance(s1.as_bytes(), s2.as_bytes());

        assert_eq!(result, Err("Strings are of different lengths"));
    }
}
