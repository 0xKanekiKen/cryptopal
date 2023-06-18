//! Repeating-key XOR

/// This function takes a plaintext and a key and returns the ciphertext in hex.
/// The key is repeated as necessary to encrypt the entire plaintext. For
/// example, if the plaintext is "Burning 'em, if you ain't quick and nimble\n\
/// I go crazy when I hear a cymbal" and the key is "ICE", then the first three
/// bytes of the plaintext will be XORed with the first three bytes of the key.
/// The next three bytes of the plaintext will be XORed with the next three
/// bytes of the key, and so on.
///
/// # Arguments
///  plaintext - the plaintext to encrypt.
///  key - the key to use to encrypt the plaintext.
///
/// # Returns
/// The ciphertext in hex.
///
/// # Examples
/// ```
/// use set1::chal5::repeating_key_xor;
///
/// let plaintext = "It's over 9000!";
/// let key = "GOKU";
/// let ciphertext = repeating_key_xor(plaintext, key);
/// assert_eq!(ciphertext, "0e3b6c2667203d30356f7265777f6a");
/// ```
pub fn repeating_key_xor(plaintext: &str, key: &str) -> String {
    let plaintext_bytes = plaintext.as_bytes();
    let key_bytes = key.as_bytes();
    let mut ciphertext = vec![];

    plaintext_bytes.iter().enumerate().for_each(|(i, byte)| {
        ciphertext.push(byte ^ key_bytes[i % key_bytes.len()]);
    });

    // convert the bytes vector to a hex string.
    bytes_to_hex(&ciphertext)
}

/// This function converts a vector of bytes to a hex string. It is used to
/// convert the ciphertext into a hex string.
///
/// # Arguments
///   bytes - a vector of bytes.
///
/// # Returns
///  A hex string.
fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut hex_string = String::new();

    // iterate over the bytes and convert them to hex.
    for byte in bytes {
        hex_string.push_str(&format!("{:02x}", byte));
    }

    hex_string
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_repeating_key_xor() {
        let plaintext = "Burning 'em, if you ain't quick and nimble\n\
                         I go crazy when I hear a cymbal";
        let key = "ICE";
        let ciphertext = repeating_key_xor(plaintext, key);
        let expected_ciphertext = "0b3637272a2b2e63622c2e69692a23693a2a3\
                                   c6324202d623d63343c2a2622632427276527\
                                   2a282b2f20430a652e2c652a3124333a653e2\
                                   b2027630c692b20283165286326302e27282\
                                   f";

        assert_eq!(ciphertext, expected_ciphertext);
    }
}
