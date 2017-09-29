use std;
use std::fmt;

use symmetric_cipher::*;

pub struct RailFenceCipherText(pub String);

impl fmt::Debug for RailFenceCipherText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RailFenceCipher:\n{}", self.0)
    }
}

struct RfCoordinates(isize, isize);

impl RfCoordinates {

    fn generate(cryptosystem: &RailFenceCipher, msg_len: usize) -> Vec<RfCoordinates> {

        fn calc_fst_coordinate(snd: isize) -> isize {
            if (snd % 4) <= 2 {
                snd % 4
            } else {
                4 - (snd % 4)
            }
        }

        (0..).take(msg_len)
            .map(|snd| RfCoordinates(calc_fst_coordinate(snd), snd) )
            .collect()
    }
}

pub struct RailFenceCipher {
    pub size: u8, // TODO: use this
}

impl SymmetricCipher<RailFenceCipherText, String> for RailFenceCipher {

    fn encrypt(&self, input: &String) -> RailFenceCipherText {
        let msg_len = input.char_indices().count();
        let list = RfCoordinates::generate(self, msg_len);
        let mut state: Vec<(RfCoordinates, char)> = list.into_iter().zip(input.chars()).collect();
        state.sort_by(|&(RfCoordinates(a, _), _), &(RfCoordinates(b, _), _)| {
            a.cmp(&b)
        });
        let cipher_text: String = state
            .into_iter()
            .map(|(_, c)| c)
            .collect();
        RailFenceCipherText(cipher_text)
    }

    fn decrypt(&self, input: &RailFenceCipherText) -> String {
        "ahoj".to_string()
    }
}

