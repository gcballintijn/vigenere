mod vigenere {
    const BASE: u8 = 'A' as u8;

    pub fn encrypt(plain_text: &str, key: &str) -> String {
        let key: Vec<char> = key.chars().collect();
        let plain_text: Vec<char> = plain_text.chars().collect();

        let mut cipher_text = String::new();
        let mut i = 0;
        for plain_char in plain_text {
            let cipher_char  = match plain_char {
                ch if ch >= 'A' && ch <= 'Z' => {
                    let distance = key[i] as u8 - BASE;
                    let plain_value = ch as u8 - BASE;
                    let cipher_value = (plain_value + distance) % 26;
                    i = (i + 1) % key.len();
                    (BASE + cipher_value) as char  
                },
                ch => ch,
            };

            cipher_text.push(cipher_char);
        }

        cipher_text
    }

    pub fn decrypt(cipher_text: &str, key: &str) -> String {
        let key: Vec<char> = key.chars().collect();
        let cipher_text: Vec<char> = cipher_text.chars().collect();

        let mut plain_text = String::new();
        let mut i = 0;
        for cipher_char in cipher_text {
            let plain_char = match cipher_char {
                ch if ch >= 'A' && ch <= 'Z' => {
                    let distance = key[i] as u8 - BASE;
                    let cipher_value = ch as u8 - BASE;
                    let plain_value = (26 + cipher_value - distance) % 26;
                    i = (i + 1) % key.len();
                    (BASE + plain_value) as char  
                },
                ch => ch,
            };

            plain_text.push(plain_char);
        }

        plain_text
    }
}

fn main() {
    use vigenere::*;

    let key = "WHYRUST";

    let plain_text = "TO EMPOWER EVERY ONE";
    let cipher_text = encrypt(plain_text, key);
    println!("Encrypting '{}' with key '{}' gives '{}'.", plain_text, key, cipher_text);


    let cipher_text = "PV CDJGPAY CMYJR KUC";
    let plain_text = decrypt(cipher_text, key);
    println!("Decrypting '{}' with key '{}' gives '{}'.", cipher_text, key, plain_text);
}

#[cfg(test)]
mod tests {
    use super::vigenere;

    #[test]
    fn encrypt_main_example() {
        let plain_text = vigenere::encrypt("TO EMPOWER EVERY ONE", "WHYRUST");
        assert_eq!(plain_text, "PV CDJGPAY CMYJR KUC");
    }

    #[test]
    fn decrypt_single_char_unit_transform() {
        let plain_text = vigenere::decrypt("A", "A");
        assert_eq!(plain_text, "A");
    }

    #[test]
    fn decrypt_multiple_chars_unit_transform() {
        let plain_text = vigenere::decrypt("ABCD", "A");
        assert_eq!(plain_text, "ABCD");
    }

    #[test]
    fn decrypt_single_char_single_step() {
        let plain_text = vigenere::decrypt("B", "B");
        assert_eq!(plain_text, "A");
    }

    #[test]
    fn decrypt_multiple_chars_single_step() {
        let plain_text = vigenere::decrypt("BCDE", "B");
        assert_eq!(plain_text, "ABCD");
    }

    #[test]
    fn decrypt_multiple_chars_different_steps() {
        let plain_text = vigenere::decrypt("BCDE", "AB");
        assert_eq!(plain_text, "BBDD");
    }

    #[test]
    fn decrypt_other_example() {
        let plain_text = vigenere::decrypt("LXFOPV EF RNHR", "LEMON");
        assert_eq!(plain_text, "ATTACK AT DAWN");
    }

    #[test]
    fn decrypt_main_example() {
        let plain_text = vigenere::decrypt("PV CDJGPAY CMYJR KUC", "WHYRUST");
        assert_eq!(plain_text, "TO EMPOWER EVERY ONE");
    }

}
