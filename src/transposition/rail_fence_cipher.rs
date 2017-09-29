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
        state.sort_by(|a,b| {
            ((a.0).0).cmp(&(b.0).0)
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

/*pub struct RailFenceCipherText {
    state: Vec<Vec<char>>,
}

impl fmt::Debug for RailFenceCipherText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RailFenceCipher\n{:?}\n{:?}\n{:?}\n", self.state[0], self.state[1], self.state[2])
    }
}

pub struct RailFenceCipher;

impl Default for RailFenceCipher {
    fn default() -> Self {
        RailFenceCipher {}
    }
}

impl SymmetricCipher<RailFenceCipherText, String> for RailFenceCipher {
    fn encrypt(&self, input: &String) -> RailFenceCipherText {
        let len = input.char_indices().count();
        let mut state = vec![
            std::iter::repeat('.')
                .take(len)
                .collect::<Vec<char>>(); 3];
        let mut row = 0;
        let mut step = 1;
        for i in 0..len {
            state[row][i] = input.chars().nth(i).unwrap();
            row = (row as i32 + step) as usize;
            if row == 2 {
                step = -1;
            } else if row == 0 {
                step = 1;
            }
        }
        RailFenceCipherText {
            state
        }
    }

    fn decrypt(&self, input: &RailFenceCipherText) -> String {
        "ahoj".to_string()
    }

}

*/