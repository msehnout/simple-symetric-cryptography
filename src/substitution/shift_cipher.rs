use std::ascii::AsciiExt;
use super::{PlainText, CipherText};
pub use symmetric_cipher::*;

pub struct ShiftCipher {
    shift: i16,
}

impl ShiftCipher {
    pub fn with_shift(shift: u8) -> Self {
        ShiftCipher {
            shift: shift as i16,
        }
    }
}

impl Default for ShiftCipher {
    fn default() -> Self {
        ShiftCipher {
            shift: 0,
        }
    }
}

impl SymmetricCipher<CipherText, PlainText> for ShiftCipher {
    fn encrypt(&self, input: &PlainText) -> CipherText {

        fn shift_ascii_value(input: char, shift: i16, base:i16, modulus:i16) -> char {
            let ascii_code = input as i16;
            let value = ascii_code - base;
            let encrypted_char = if (value + shift) >= 0 {
                (value + shift) % modulus
            } else {
                (value + shift + modulus) % modulus
            };
            (encrypted_char + base) as u8 as char
        }

        input.clone()
            .to_lowercase()
            .chars()
            .map(|c| {
                if c.is_ascii_digit() {
                    shift_ascii_value(c, self.shift, 48, 10)
                } else if c.is_ascii_lowercase() {
                    shift_ascii_value(c, self.shift, 97, 26)
                } else {
                    c
                }
            })
            .collect::<String>()
            .to_uppercase()
    }

    fn decrypt(&self, input: &CipherText) -> PlainText {
        ShiftCipher {shift: -self.shift}
            .encrypt(&input)
            .to_lowercase()
    }
}
