use std::collections::HashMap;

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
