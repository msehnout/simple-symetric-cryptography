#![feature(ascii_ctype)]

mod symmetric_cipher;
mod shift_cipher;

use shift_cipher::*;

fn main() {
    let plain_text = "hello, world! 9876".to_string();
    let cipher_text = ShiftCipher::with_shift(3).encrypt(&plain_text);
    let decrypted_text = ShiftCipher::with_shift(3).decrypt(&cipher_text);
    println!("Plain text: {}", plain_text);
    println!("Cipher text: {}", cipher_text);
    println!("Decrypted text: {}", decrypted_text);
}
