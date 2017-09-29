use std;
use std::fmt;

use symmetric_cipher::*;

pub struct RailFenceCipherText {
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

