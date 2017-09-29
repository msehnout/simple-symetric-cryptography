#![feature(ascii_ctype)]

mod substitution;
mod transposition;
mod symmetric_cipher;

use substitution::shift_cipher::*;
use transposition::rail_fence_cipher::*;

fn main() {
    let plain_text = "hello, world! 9876".to_string();
    let cipher_text = ShiftCipher::with_shift(3).encrypt(&plain_text);
    let decrypted_text = ShiftCipher::with_shift(3).decrypt(&cipher_text);
    println!("Plain text: {}", plain_text);
    println!("Cipher text: {}", cipher_text);
    println!("Decrypted text: {}", decrypted_text);

    let c: RailFenceCipherText = RailFenceCipherText("ahoj".to_string());
    println!("{:?}", c);

    let s = RailFenceCipher { size: 3 };
    let o = s.encrypt(&"ahojah".to_string());
    println!("Encrypted:{:?}", o);
    let a = s.decrypt(&o);
    println!("Decrypted:{:?}", a);
    /*let plain_text = "ahojjakje".to_string();
    let cryptosystem = RailFenceCipher::default();
    let output = cryptosystem.encrypt(&plain_text);
    println!("Encrypted:{:?}", output);*/

}
