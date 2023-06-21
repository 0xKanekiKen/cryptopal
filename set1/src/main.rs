use set1::{
    chal3::single_byte_xor_character, chal4::detect_single_char_xor,
    chal6::break_repeating_key_xor, chal7::aes_in_ecb_mode, chal8::detect_aes_in_ecb_mode,
};

fn main() {
    single_byte_xor_character();
    detect_single_char_xor();
    break_repeating_key_xor();
    aes_in_ecb_mode();
    detect_aes_in_ecb_mode();
}
