/// Challenge: Fixed XOR. The method takes two equal length buffers and produces
/// their XOR combination. If the buffers are not equal length, then an error
/// is returned. The method returns a hex string.
///
/// # Examples
/// ```
/// use set1::chal2::fixed_xor;
/// let buffer1 = "1c0111001f010100061a024b53535009181c";
/// let buffer2 = "686974207468652062756c6c277320657965";
/// let xor = fixed_xor(buffer1, buffer2).unwrap();
///
/// assert_eq!(xor, "746865206b696420646f6e277420706c6179");
/// ```
pub fn fixed_xor<'a>(buffer1: &str, buffer2: &str) -> Result<String, &'a str> {
    if buffer1.len() != buffer2.len() {
        return Err("Buffers are not the same length.");
    }

    // iterate over both the hex strings buffers and xor them.
    let xor = buffer1
        .chars()
        .zip(buffer2.chars())
        .map(|(a, b)| {
            // convert the characters into bytes.
            let a = a.to_digit(16).unwrap() as u8;
            let b = b.to_digit(16).unwrap() as u8;

            // xor the bytes.
            a ^ b
        })
        .map(|xor_char| format!("{:x}", xor_char))
        .collect::<String>();

    Ok(xor)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fixed_xor() {
        // --------------------- test valid hex strings ---------------------//

        let buffer1 = "1c0111001f010100061a024b53535009181c";
        let buffer2 = "686974207468652062756c6c277320657965";
        let xor = fixed_xor(buffer1, buffer2).unwrap();

        assert_eq!(xor, "746865206b696420646f6e277420706c6179");

        //----------- test buffers of different lengths -----------//

        let buffer1 = "1c0111001f010100061a024b53535009181c";
        let buffer2 = "686974207468652062756c6c277320657965746865206b696420646f6e277420706c6179";
        let xor = fixed_xor(buffer1, buffer2);

        assert_eq!(xor, Err("Buffers are not the same length."));
    }
}
