mod iterator;

use iterator::{Encrypt, Decrypt};

pub struct Vigenere(String);

impl Vigenere {
    pub fn new(key: &str) -> Self {
        Self(key.to_owned())
    }

    pub fn encrypt(&self, plain_text: &str) -> String {
        plain_text.chars().encrypt(&self.0).collect()
    }
    
    pub fn decrypt(&self, cipher_text: &str) -> String {
        cipher_text.chars().decrypt(&self.0).collect()
    }    
}

#[cfg(test)]
mod tests {
    use super::Vigenere;

    #[test]
    fn encrypt_main_example() {
        let encrypter = Vigenere::new("WHYRUST");

        let plain_text = encrypter.encrypt("TO EMPOWER EVERYONE");

        assert_eq!(plain_text, "PV CDJGPAY CMYJRKUC");
    }

    #[test]
    fn decrypt_single_char_unit_transform() {
        let decrypter = Vigenere::new("A");

        let plain_text = decrypter.decrypt("Q");

        assert_eq!(plain_text, "Q");
    }

    #[test]
    fn decrypt_multiple_chars_unit_transform() {
        let decrypter = Vigenere::new("A");

        let plain_text = decrypter.decrypt("QRST");

        assert_eq!(plain_text, "QRST");
    }

    #[test]
    fn decrypt_single_char_single_step() {
        let decrypter = Vigenere::new("B");

        let plain_text = decrypter.decrypt("D");

        assert_eq!(plain_text, "C");
    }

    #[test]
    fn decrypt_multiple_chars_single_step() {
        let decrypter = Vigenere::new("B");

        let plain_text = decrypter.decrypt("DEFG");

        assert_eq!(plain_text, "CDEF");
    }

    #[test]
    fn decrypt_multiple_chars_different_steps() {
        let decrypter = Vigenere::new("AB");

        let plain_text = decrypter.decrypt("BCDE");

        assert_eq!(plain_text, "BBDD");
    }

    #[test]
    fn decrypt_other_example() {
        let decrypter = Vigenere::new("LEMON");

        let plain_text = decrypter.decrypt("LXFOPV EF RNHR");

        assert_eq!(plain_text, "ATTACK AT DAWN");
    }

    #[test]
    fn decrypt_main_example() {
        let decrypter = Vigenere::new("WHYRUST");

        let plain_text = decrypter.decrypt("PV CDJGPAY CMYJR KUC");

        assert_eq!(plain_text, "TO EMPOWER EVERY ONE");
    }

}
