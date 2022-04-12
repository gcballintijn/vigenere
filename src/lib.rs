#![warn(missing_docs)]

//! Implementation of the Vigenère cipher.
//!
//! For some background information, see the [Wikipedia entry](https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher).
//!
//! ### Encyption example
//! ```rust
//! # fn main() {
//!     let cypher = vigenere::Vigenere::new("WHYRUST");
//!     let cipher_text = cypher.encrypt("TO EMPOWER EVERYONE");
//!     println!("Encrypting gives '{}'.", cipher_text);
//! # }
//! ```
//! Resulting output: `Encrypting gives 'PV CDJGPAY CMYJRKUC'.`
//! 
//! ### Decyption example
//! ```rust
//! # fn main() {
//!     let cypher = vigenere::Vigenere::new("WHYRUST");
//!     let plain_text = cypher.decrypt("PV CDJGPAY CMYJRKUC");
//!     println!("Decrypting gives '{}'.", plain_text);
//! # }
//! ```
//! Resulting output: `Decrypting gives 'TO EMPOWER EVERYONE'.`

pub mod iterator;

use iterator::{Encrypt, Decrypt};

/// Struct to encrypt or decrypt a string slice using the Vigenère cipher. 
pub struct Vigenere(String);

impl Vigenere {
    /// Creates and returns a configured Vigenère encryption/decryption object.  
    pub fn new(key: &str) -> Self {
        Self(key.to_owned())
    }

    /// Encrypts the provided plain text and returns the resulting cipher text.
    pub fn encrypt(&self, plain_text: &str) -> String {
        plain_text.chars().encrypt(&self.0, None, None).collect()
    }
    
    /// Decrypts the provided cipher text and returns the resulting plain text.
    pub fn decrypt(&self, cipher_text: &str) -> String {
        cipher_text.chars().decrypt(&self.0, None, None).collect()
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
