use crate::chal6::base64_to_bytes;
use openssl::symm::{decrypt, Cipher};
use std::fs;

pub fn aes_in_ecb_mode() {
    // read the file.
    let cipher: String = fs::read_to_string("set1/data/chal7/7.txt")
        .unwrap()
        .parse::<String>()
        .unwrap();

    // replace the newline characters with empty strings.
    let cipher = cipher.replace("\n", "");
    let cipher_bytes = base64_to_bytes(&cipher).unwrap();

    // decrypt the ciphertext.
    let key = "YELLOW SUBMARINE";
    let decrypt_bytes =
        decrypt(Cipher::aes_128_ecb(), key.as_bytes(), None, &cipher_bytes).unwrap();

    println!(
        "Decryption: {}",
        String::from_utf8(decrypt_bytes.clone()).unwrap()
    );

    // save the decrypted text to a file.
    fs::write("set1/data/chal7/7_decryption.txt", decrypt_bytes).unwrap();
}
