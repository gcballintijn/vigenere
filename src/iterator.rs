#![warn(missing_docs)]

//! Iterator-based implementation of the Vigenère cipher.

/// Type of operation of the character iterator.
enum VigenereMode {
    /// Decrypt the stream of characters.
    Decrypt,
    /// Encrypt the stream of characters.
    Encrypt,
}

/// Method of dealing with the case of letters.
#[derive(PartialEq)]
pub enum ForceCase {
    /// Keep the case of the characters.
    Keep,
    /// Force the character to lower case.  
    ToLower,
    /// Force the character to upper case.
    ToUpper,
}

/// Method of dealing with non-letter characters.
#[derive(PartialEq)]
pub enum NonLetterMode {
    /// Keep non-letter characters.
    Keep,
    /// Skip (*i.e.*, remove) non-letter characters.
    Skip,
}

/// Iterator, over characters, that encrypts or decrypts the character using the Vigenère cipher.
pub struct VigenereIterator<I>
where
    I: Iterator<Item = char>,
{
    mode: VigenereMode,
    distances: Vec<u8>,
    force_case: ForceCase,
    none_letter_mode: NonLetterMode,
    index: usize,
    iter: I,
}

impl<I> VigenereIterator<I>
where
    I: Iterator<Item = char>,
{
    const UPPER_BASE: u8 = 'A' as u8;
    const LOWER_BASE: u8 = 'a' as u8;

    fn new(mode: VigenereMode, key: &str, iter: I) -> Self {
        let distances = key
            .chars()
            .map(|ch| ch.to_uppercase().next().unwrap())
            .map(|ch| ch as u8 - Self::UPPER_BASE)
            .collect::<Vec<u8>>();
        Self {
            mode,
            distances,
            force_case: ForceCase::Keep,
            none_letter_mode: NonLetterMode::Keep,
            index: 0,
            iter,
        }
    }

    fn with_force_case(self, force_case: ForceCase) -> Self {
        Self {
            force_case,
            ..self
        }
    }

    fn with_none_letter_mode(self, none_letter_mode: NonLetterMode) -> Self {
        Self {
            none_letter_mode,
            ..self
        }
    }
}

impl<I> Iterator for VigenereIterator<I>
where
    I: Iterator<Item = char>,
{
    type Item = char;

    fn next(&mut self) -> Option<char> {
        loop {
            break match self.iter.next() {
                Some(ch) if ch >= 'A' && ch <= 'Z' => {
                    let distance = self.distances[self.index];
                    self.index = (self.index + 1) % self.distances.len();
                    let input = ch as u8 - Self::UPPER_BASE;
                    let output = match self.mode {
                        VigenereMode::Encrypt => (input + distance) % 26,
                        VigenereMode::Decrypt => (26 + input - distance) % 26,
                    };
                    Some(if self.force_case == ForceCase::ToLower {
                        Self::LOWER_BASE + output
                    } else {
                        Self::UPPER_BASE + output
                    } as char)
                }
    
                Some(ch) if ch >= 'a' && ch <= 'z' => {
                    let distance = self.distances[self.index];
                    self.index = (self.index + 1) % self.distances.len();
                    let input = ch as u8 - Self::LOWER_BASE;
                    let output = match self.mode {
                        VigenereMode::Encrypt => (input + distance) % 26,
                        VigenereMode::Decrypt => (26 + input - distance) % 26,
                    };
                    Some(if self.force_case == ForceCase::ToUpper {
                        Self::UPPER_BASE + output
                    } else {
                        Self::LOWER_BASE + output
                    } as char)
                }
    
                Some(ch) => if self.none_letter_mode == NonLetterMode::Skip {
                    continue;
                } else {
                    Some(ch)
                }
                None => None,
            };
        }
    }
}

/// Encryption trait for a character iterator.
pub trait Encrypt: Iterator<Item = char> + Sized {
    /// Encrypt characters using the specified key and configuration.
    fn encrypt(
        self,
        key: &str,
        force_case: Option<ForceCase>,
        none_letter_mode: Option<NonLetterMode>,
    ) -> VigenereIterator<Self>;
}

/// Decryption trait for a character iterator.
pub trait Decrypt: Iterator<Item = char> + Sized {
    /// Decrypt characters using the specified key and configuration.
    fn decrypt(
        self,
        key: &str,
        force_case: Option<ForceCase>,
        none_letter_mode: Option<NonLetterMode>,
    ) -> VigenereIterator<Self>;
}

impl<I> Encrypt for I
where
    I: Iterator<Item = char>,
{
    fn encrypt(
        self,
        key: &str,
        force_case: Option<ForceCase>,
        none_letter_mode: Option<NonLetterMode>,
    ) -> VigenereIterator<I> {
        let mut vig_iter = VigenereIterator::new(VigenereMode::Encrypt, key, self);
        if let Some(force_case) = force_case {
            vig_iter = vig_iter.with_force_case(force_case);
        }
        if let Some(none_letter_mode) = none_letter_mode {
            vig_iter = vig_iter.with_none_letter_mode(none_letter_mode);
        }
        vig_iter
    }
}

impl<I> Decrypt for I
where
    I: Iterator<Item = char>,
{
    fn decrypt(
        self,
        key: &str,
        force_case: Option<ForceCase>,
        none_letter_mode: Option<NonLetterMode>,
    ) -> VigenereIterator<I> {
        let mut vig_iter = VigenereIterator::new(VigenereMode::Decrypt, key, self);
        if let Some(force_case) = force_case {
            vig_iter = vig_iter.with_force_case(force_case);
        }
        if let Some(none_letter_mode) = none_letter_mode {
            vig_iter = vig_iter.with_none_letter_mode(none_letter_mode);
        }
        vig_iter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_encrypt_upper_key_upper_input() {
        let s = "HI";
        let iter_in = s.chars();
        let mut iter_out = iter_in.encrypt("A", None, None);
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn identity_encrypt_lower_key_upper_input() {
        let s = "HI";
        let iter_in = s.chars();
        let mut iter_out = iter_in.encrypt("a", None, None);
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn identity_encrypt_upper_key_lower_input() {
        let s = "hi";
        let iter_in = s.chars();
        let mut iter_out = iter_in.encrypt("A", None, None);
        assert_eq!(iter_out.next(), Some('h'));
        assert_eq!(iter_out.next(), Some('i'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn identity_encrypt_lower_key_lower_input() {
        let s = "hi";
        let iter_in = s.chars();
        let mut iter_out = iter_in.encrypt("a", None, None);
        assert_eq!(iter_out.next(), Some('h'));
        assert_eq!(iter_out.next(), Some('i'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn minimal_encrypt_upper_key_upper_input() {
        let s = "HI";
        let iter_in = s.chars();
        let mut iter_out = iter_in.encrypt("B", None, None);
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), Some('J'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn minimal_encrypt_lower_key_upper_input() {
        let s = "HI";
        let iter_in = s.chars();
        let mut iter_out = iter_in.encrypt("b", None, None);
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), Some('J'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn minimal_encrypt_upper_key_lower_input() {
        let s = "hi";
        let iter_in = s.chars();
        let mut iter_out = iter_in.encrypt("B", None, None);
        assert_eq!(iter_out.next(), Some('i'));
        assert_eq!(iter_out.next(), Some('j'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn minimal_encrypt_lower_key_lower_input() {
        let s = "hi";
        let iter_in = s.chars();
        let mut iter_out = iter_in.encrypt("b", None, None);
        assert_eq!(iter_out.next(), Some('i'));
        assert_eq!(iter_out.next(), Some('j'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn bigger_encrypt() {
        let s = "HiHi";
        let iter_in = s.chars();
        let mut iter_out = iter_in.encrypt("ABC", None, None);
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('j'));
        assert_eq!(iter_out.next(), Some('J'));
        assert_eq!(iter_out.next(), Some('i'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn bigger_encrypt_force_lower() {
        let s = "HiHi";
        let iter_in = s.chars();
        let mut iter_out = iter_in.encrypt("ABC", Some(ForceCase::ToLower), None);
        assert_eq!(iter_out.next(), Some('h'));
        assert_eq!(iter_out.next(), Some('j'));
        assert_eq!(iter_out.next(), Some('j'));
        assert_eq!(iter_out.next(), Some('i'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn bigger_encrypt_force_upper() {
        let s = "HiHi";
        let iter_in = s.chars();
        let mut iter_out = iter_in.encrypt("ABC", Some(ForceCase::ToUpper), None);
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('J'));
        assert_eq!(iter_out.next(), Some('J'));
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn bigger_encrypt_skip_nonletters() {
        let s = "H-i H+i";
        let iter_in = s.chars();
        let mut iter_out = iter_in.encrypt("ABC", None, Some(NonLetterMode::Skip));
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('j'));
        assert_eq!(iter_out.next(), Some('J'));
        assert_eq!(iter_out.next(), Some('i'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn identity_decrypt_upper_key_upper_input() {
        let s = "HI";
        let iter_in = s.chars();
        let mut iter_out = iter_in.decrypt("A", None, None);
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn identity_decrypt_lower_key_upper_input() {
        let s = "HI";
        let iter_in = s.chars();
        let mut iter_out = iter_in.decrypt("a", None, None);
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn identity_decrypt_upper_key_lower_input() {
        let s = "hi";
        let iter_in = s.chars();
        let mut iter_out = iter_in.decrypt("A", None, None);
        assert_eq!(iter_out.next(), Some('h'));
        assert_eq!(iter_out.next(), Some('i'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn identity_decrypt_lower_key_lower_input() {
        let s = "hi";
        let iter_in = s.chars();
        let mut iter_out = iter_in.decrypt("a", None, None);
        assert_eq!(iter_out.next(), Some('h'));
        assert_eq!(iter_out.next(), Some('i'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn minimal_decrypt_upper_key_upper_input() {
        let s = "IJ";
        let iter_in = s.chars();
        let mut iter_out = iter_in.decrypt("B", None, None);
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn minimal_decrypt_lower_key_upper_input() {
        let s = "IJ";
        let iter_in = s.chars();
        let mut iter_out = iter_in.decrypt("b", None, None);
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn minimal_decrypt_upper_key_lower_input() {
        let s = "ij";
        let iter_in = s.chars();
        let mut iter_out = iter_in.decrypt("B", None, None);
        assert_eq!(iter_out.next(), Some('h'));
        assert_eq!(iter_out.next(), Some('i'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn minimal_decrypt_lower_key_lower_input() {
        let s = "ij";
        let iter_in = s.chars();
        let mut iter_out = iter_in.decrypt("b", None, None);
        assert_eq!(iter_out.next(), Some('h'));
        assert_eq!(iter_out.next(), Some('i'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn bigger_decrypt() {
        let s = "HjJi";
        let iter_in = s.chars();
        let mut iter_out = iter_in.decrypt("ABC", None, None);
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('i'));
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('i'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn bigger_decrypt_force_lower() {
        let s = "HjJi";
        let iter_in = s.chars();
        let mut iter_out = iter_in.decrypt("ABC", Some(ForceCase::ToLower), None);
        assert_eq!(iter_out.next(), Some('h'));
        assert_eq!(iter_out.next(), Some('i'));
        assert_eq!(iter_out.next(), Some('h'));
        assert_eq!(iter_out.next(), Some('i'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn bigger_decrypt_force_upper() {
        let s = "HjJi";
        let iter_in = s.chars();
        let mut iter_out = iter_in.decrypt("ABC", Some(ForceCase::ToUpper), None);
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn bigger_decrypt_skip_nonletters() {
        let s = "H.j^J,i";
        let iter_in = s.chars();
        let mut iter_out = iter_in.decrypt("ABC", None, Some(NonLetterMode::Skip));
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('i'));
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('i'));
        assert_eq!(iter_out.next(), None);
    }
}
