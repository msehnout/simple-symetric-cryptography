pub trait SymmetricCipher<CipherText, PlainText> {
    fn encrypt(&self, input: &PlainText) -> CipherText;
    fn decrypt(&self, input: &CipherText) -> PlainText;
}
