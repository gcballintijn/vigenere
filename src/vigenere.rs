pub struct Vigenere {
    distances: Vec<u8>,
}

impl Vigenere {
    const BASE: u8 = 'A' as u8;

    pub fn new(key: &str) -> Self {
        Self {
            distances: key.chars()
                .map(|ch| ch as u8 - Self::BASE)
                .collect(),
        }
    }

    pub fn encrypt(&self, plain_text: &str) -> String {
        let plain_text: Vec<char> = plain_text.chars().collect();
    
        let mut cipher_text = String::new();
        let mut key = self.distances.iter().cycle();
        for plain_char in plain_text {
            let cipher_char  = match plain_char {
                ch if ch >= 'A' && ch <= 'Z' => {
                    let distance = *key.next().unwrap();
                    let plain_value = ch as u8 - Self::BASE;
                    let cipher_value = (plain_value + distance) % 26;
                    (Self::BASE + cipher_value) as char  
                },
                ch => ch,
            };
    
            cipher_text.push(cipher_char);
        }
    
        cipher_text
    }
    
    pub fn decrypt(&self, cipher_text: &str) -> String {
        let cipher_text: Vec<char> = cipher_text.chars().collect();
    
        let mut plain_text = String::new();
        let mut key = self.distances.iter().cycle();
        for cipher_char in cipher_text {
            let plain_char = match cipher_char {
                ch if ch >= 'A' && ch <= 'Z' => {
                    let distance = *key.next().unwrap();
                    let cipher_value = ch as u8 - Self::BASE;
                    let plain_value = (26 + cipher_value - distance) % 26;
                    (Self::BASE + plain_value) as char  
                },
                ch => ch,
            };
    
            plain_text.push(plain_char);
        }
    
        plain_text
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
