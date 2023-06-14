/// This method converts a hex string into raw bytes. The hex string should be
/// even in length, otherwise it will return an error. Two hex characters can be
/// converted into a single byte. The first character is the most significant
/// nibble, and the second character is the least significant nibble.
///
/// # Examples
/// ```
/// use set1::chal1::hex_to_bytes;
/// let hex = "4d";
/// let bytes = hex_to_bytes(hex);
/// assert_eq!(bytes, Ok(vec![0x4d]));
/// ```
///
/// ```
/// use set1::chal1::hex_to_bytes;
/// let hex = "4d61";
/// let bytes = hex_to_bytes(hex);
/// assert_eq!(bytes, Ok(vec![0x4d, 0x61]));    
/// ```
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, &str> {
    let mut bytes_vec = Vec::new();
    let mut chars = hex.chars();
    while let Some(c1) = chars.next() {
        let c2 = chars
            .next()
            .ok_or("Invalid hex string, it should be even in length")?;
        let byte = match (c1.to_digit(16), c2.to_digit(16)) {
            (Some(h1), Some(h2)) => (h1 << 4 | h2) as u8,
            _ => return Err("Invalid hex string, it should only contain 0-9 and a-f"),
        };
        bytes_vec.push(byte);
    }

    Ok(bytes_vec)
}

/// This method converts raw bytes into a base64 string. The base64 string is
/// composed of 64 characters. The first 26 characters are uppercase letters,
/// the next 26 characters are lowercase letters, the next 10 characters are
/// digits, and the last two characters are '+' and '/'.
///
/// The base64 string is composed of 4 characters for every 3 bytes. If the
/// number of bytes is not divisible by 3, then the last 4 characters will be
/// padded with '='. The first character is the first 6 bits of the first byte,
/// the second character is the last 2 bits of the first byte and the first
/// 4 bits of the second byte, the third character is the last 4 bits of the
/// second byte and the first 2 bits of the third byte, and the fourth character
/// is the last 6 bits of the third byte.
///
/// If there are only 2 bytes, then the third character will be padded with '='.
/// If there is only 1 byte, then the second and third characters will be
/// padded with '='.
///
/// # Examples
/// ```
/// use set1::chal1::bytes_to_base64;
/// let bytes = vec![0x4d, 0x61, 0x6e];
/// let b64 = bytes_to_base64(&bytes);
///
/// assert_eq!(b64, "TWFu");
/// ```
///
/// ```
/// use set1::chal1::bytes_to_base64;
/// let bytes = vec![0x4d, 0x61];
/// let b64 = bytes_to_base64(&bytes);
///
/// assert_eq!(b64, "TWE=");
/// ```
///
/// ```
/// use set1::chal1::bytes_to_base64;
/// let bytes = vec![0x4d];
/// let b64 = bytes_to_base64(&bytes);
///
/// assert_eq!(b64, "TQ==");
/// ```
///
/// ```
/// use set1::chal1::bytes_to_base64;
/// let bytes = vec![0x4d, 0x61, 0x6e, 0x64];
/// let b64 = bytes_to_base64(&bytes);
///
/// assert_eq!(b64, "TWFuZA==");
/// ```
pub fn bytes_to_base64(bytes: &[u8]) -> String {
    // base64 charset.
    const CHARSET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789+/";

    let mut b64 = String::new();

    // iterating over chunks of 3 bytes.
    for chunks in bytes.chunks_exact(3) {
        // first sextet is the first 6 bits of the first byte.
        let first_sextet = chunks[0] >> 2;

        // second sextet is the last 2 bits of the first byte and the first 4
        // bits of the second byte.
        let second_sextet = (chunks[0] & 0b0000_0011) << 4 | chunks[1] >> 4;

        // third sextet is the last 4 bits of the second byte and the first 2
        // bits of the third byte.
        let third_sextet = (chunks[1] & 0b0000_1111) << 2 | chunks[2] >> 6;

        // fourth sextet is the last 6 bits of the third byte.
        let fourth_sextet = chunks[2] & 0b0011_1111;

        // pushing the characters into the base64 string.
        b64.push(CHARSET[first_sextet as usize] as char);
        b64.push(CHARSET[second_sextet as usize] as char);
        b64.push(CHARSET[third_sextet as usize] as char);
        b64.push(CHARSET[fourth_sextet as usize] as char);
    }

    // checking for remainder.
    let remainder = bytes.chunks_exact(3).remainder();
    let _ = match remainder.len() {
        1 => {
            // if there is only 1 byte, then the second and third characters
            // will be padded with '='.
            let first_sextet = remainder[0] >> 2;
            let second_sextet = (remainder[0] & 0b0000_0011) << 4;
            b64.push(CHARSET[first_sextet as usize] as char);
            b64.push(CHARSET[second_sextet as usize] as char);
            b64.push('=');
            b64.push('=');
        }
        2 => {
            // if there are only 2 bytes, then the third character will be
            // padded with '='.
            let first_sextet = remainder[0] >> 2;
            let second_sectet = (remainder[0] & 0b0000_0011) << 4 | remainder[1] >> 4;
            let third_sextet = (remainder[1] & 0b0000_1111) << 2;
            b64.push(CHARSET[first_sextet as usize] as char);
            b64.push(CHARSET[second_sectet as usize] as char);
            b64.push(CHARSET[third_sextet as usize] as char);
            b64.push('=');
        }
        _ => (),
    };

    b64
}

/// This method converts a hex string into a base64 string. It first converts
/// the hex string into bytes, and then converts the bytes into a base64. If
/// the hex string is invalid, then it will return an error.
pub fn hex_to_base64(hex: &str) -> Result<String, &str> {
    let bytes = hex_to_bytes(hex)?;
    Ok(bytes_to_base64(&bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_bytes() {
        //--------------------- test valid hex strings ---------------------//

        let hex = "33100EF27C61C9FCD2BF";
        let expected_bytes_vector =
            vec![0x33, 0x10, 0x0E, 0xF2, 0x7C, 0x61, 0xC9, 0xFC, 0xD2, 0xBF];
        let result_bytes_vector = hex_to_bytes(hex);

        assert_eq!(result_bytes_vector, Ok(expected_bytes_vector));

        //----------------- test big hex string ----------------------------//

        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected_bytes_vector = vec![
            0x49, 0x27, 0x6d, 0x20, 0x6b, 0x69, 0x6c, 0x6c, 0x69, 0x6e, 0x67, 0x20, 0x79, 0x6f,
            0x75, 0x72, 0x20, 0x62, 0x72, 0x61, 0x69, 0x6e, 0x20, 0x6c, 0x69, 0x6b, 0x65, 0x20,
            0x61, 0x20, 0x70, 0x6f, 0x69, 0x73, 0x6f, 0x6e, 0x6f, 0x75, 0x73, 0x20, 0x6d, 0x75,
            0x73, 0x68, 0x72, 0x6f, 0x6f, 0x6d,
        ];

        let result_bytes_vector = hex_to_bytes(hex);
        assert_eq!(result_bytes_vector, Ok(expected_bytes_vector));

        //------------------- test invalid hex string ----------------------//

        let hex = "33100EF27C61C9FCD2BFG";
        let result_bytes_vector = hex_to_bytes(hex);

        assert_eq!(
            result_bytes_vector,
            Err("Invalid hex string, it should be even in length")
        );
        // catch panic
    }

    #[test]
    fn test_hex_to_base64() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected_b64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let result_b64 = hex_to_base64(hex);

        assert_eq!(result_b64, Ok(expected_b64.to_string()));
    }
}
