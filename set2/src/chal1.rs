/// # PKCS#7 padding: The plaintext is padded with a number of bytes such that
/// the length of the plaintext is a multiple of the block size. The value of
/// each byte of the padding is equal to the number of bytes of padding.
///
/// Arguments:
/// * `plaintext` - The plaintext to be padded.
/// * `final_length` - The length of the padded plaintext.
pub fn apply_padding(plaintext: String, final_length: usize) -> String {
    if final_length < plaintext.len() {
        panic!("Final length is less than plaintext length");
    }
    let num_pad = final_length - plaintext.len();

    // serialize the plaintext first.
    let mut plaintext_serialized = plaintext.as_bytes().to_vec();

    // add `num_pad` pads to the serialized plaintext.
    plaintext_serialized.append(&mut vec![num_pad as u8; num_pad]);

    // deserialize padded plaintext.
    let padded_plaintext = String::from_utf8(plaintext_serialized)
        .expect("Deserialization of padded plaintext failed");

    padded_plaintext
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_apply_padding() {
        let plaintext = String::from("YELLOW SUBMARINE");
        let padded_plaintext = apply_padding(plaintext, 20);
        assert_eq!(padded_plaintext, "YELLOW SUBMARINE\x04\x04\x04\x04");
    }
}
