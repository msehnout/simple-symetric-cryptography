pub type PlainText = String;
pub type CipherText = String;

pub trait SymmetricCipher {
    fn encrypt(&self, input: &PlainText) -> CipherText;
    fn decrypt(&self, input: &CipherText) -> PlainText;
}