//! Iterator-based implementation of the Vigenère cipher.

/// Type of operation of the iterator.
enum VigenereMode {
    Encrypt,
    Decrypt,
}

/// Method of dealing with the case of letters.
#[derive(PartialEq)]
pub enum ForceCase {
    Keep,
    ToLower,
    ToUpper,
}

/// Iterator, over characters, that encrypts or decrypts the character using the Vigenère cipher. 
pub struct VigenereIterator<I>
    where I: Iterator<Item = char>
{
    mode: VigenereMode,
    distances: Vec<u8>,
    force_case: ForceCase,
    index: usize,
    iter: I,
}

impl<I> VigenereIterator<I>
    where I: Iterator<Item = char>
{
    const UPPER_BASE: u8 = 'A' as u8;
    const LOWER_BASE: u8 = 'a' as u8;

    fn new(mode: VigenereMode, key: &str, iter: I) -> Self {
        let distances = key.chars()
            .map(|ch| ch.to_uppercase().next().unwrap())
            .map(|ch| ch as u8 - Self::UPPER_BASE)
            .collect::<Vec<u8>>();
        Self {
            mode,
            distances,
            force_case: ForceCase::Keep,
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
}

impl<I> Iterator for VigenereIterator<I>
    where I: Iterator<Item = char>
{
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self.iter.next() {
            Some(ch) if ch >= 'A' && ch <= 'Z' => {
                let distance = self.distances[self.index];
                self.index = (self.index + 1) % self.distances.len();
                let input = ch as u8 - Self::UPPER_BASE;
                let output = match self.mode {
                    VigenereMode::Encrypt => (input + distance) % 26,
                    VigenereMode::Decrypt => (26 + input - distance) % 26,
                };
                Some(
                    if self.force_case == ForceCase::ToLower {
                        Self::LOWER_BASE + output  
                    } else {
                        Self::UPPER_BASE + output
                    } as char
                )
            },

            Some(ch) if ch >= 'a' && ch <= 'z' => {
                let distance = self.distances[self.index];
                self.index = (self.index + 1) % self.distances.len();
                let input = ch as u8 - Self::LOWER_BASE;
                let output = match self.mode {
                    VigenereMode::Encrypt => (input + distance) % 26,
                    VigenereMode::Decrypt => (26 + input - distance) % 26,
                };
                Some(
                    if self.force_case == ForceCase::ToUpper {
                        Self::UPPER_BASE + output
                    } else {
                        Self::LOWER_BASE + output  
                    } as char
                )
            },

            Some(ch) => Some(ch),
            None => None
        }
    }
}

pub trait Encrypt : Iterator<Item = char> + Sized {
    fn encrypt(self, key: &str, force_case: Option<ForceCase>) -> VigenereIterator<Self>;
}

pub trait Decrypt : Iterator<Item = char> + Sized {
    fn decrypt(self, key: &str, force_case: Option<ForceCase>) -> VigenereIterator<Self>;
}

impl<I> Encrypt for I
    where I: Iterator<Item = char>
{
    fn encrypt(self, key: &str, force_case: Option<ForceCase>) -> VigenereIterator<I> {
        let vig_iter = VigenereIterator::new(VigenereMode::Encrypt, key, self);
        match force_case {
            Some(force_case) => vig_iter.with_force_case(force_case),
            None => vig_iter
        }
    }
}

impl<I> Decrypt for I
    where I: Iterator<Item = char>
{
    fn decrypt(self, key: &str, force_case: Option<ForceCase>) -> VigenereIterator<I> {
        let vig_iter = VigenereIterator::new(VigenereMode::Decrypt, key, self);
        match force_case {
            Some(force_case) => vig_iter.with_force_case(force_case),
            None => vig_iter
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_encrypt_upper_key_upper_input() {
        let s = "HI";
        let iter_in = s.chars();
        let mut iter_out = iter_in.encrypt("A", None);
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn identity_encrypt_lower_key_upper_input() {
        let s = "HI";
        let iter_in = s.chars();
        let mut iter_out = iter_in.encrypt("a", None);
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn identity_encrypt_upper_key_lower_input() {
        let s = "hi";
        let iter_in = s.chars();
        let mut iter_out = iter_in.encrypt("A", None);
        assert_eq!(iter_out.next(), Some('h'));
        assert_eq!(iter_out.next(), Some('i'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn identity_encrypt_lower_key_lower_input() {
        let s = "hi";
        let iter_in = s.chars();
        let mut iter_out = iter_in.encrypt("a", None);
        assert_eq!(iter_out.next(), Some('h'));
        assert_eq!(iter_out.next(), Some('i'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn minimal_encrypt_upper_key_upper_input() {
        let s = "HI";
        let iter_in = s.chars();
        let mut iter_out = iter_in.encrypt("B", None);
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), Some('J'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn minimal_encrypt_lower_key_upper_input() {
        let s = "HI";
        let iter_in = s.chars();
        let mut iter_out = iter_in.encrypt("b", None);
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), Some('J'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn minimal_encrypt_upper_key_lower_input() {
        let s = "hi";
        let iter_in = s.chars();
        let mut iter_out = iter_in.encrypt("B", None);
        assert_eq!(iter_out.next(), Some('i'));
        assert_eq!(iter_out.next(), Some('j'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn minimal_encrypt_lower_key_lower_input() {
        let s = "hi";
        let iter_in = s.chars();
        let mut iter_out = iter_in.encrypt("b", None);
        assert_eq!(iter_out.next(), Some('i'));
        assert_eq!(iter_out.next(), Some('j'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn bigger_encrypt() {
        let s = "HiHi";
        let iter_in = s.chars();
        let mut iter_out = iter_in.encrypt("ABC", None);
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
        let mut iter_out = iter_in.encrypt("ABC", Some(ForceCase::ToLower));
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
        let mut iter_out = iter_in.encrypt("ABC", Some(ForceCase::ToUpper));
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('J'));
        assert_eq!(iter_out.next(), Some('J'));
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn identity_decrypt_upper_key_upper_input() {
        let s = "HI";
        let iter_in = s.chars();
        let mut iter_out = iter_in.decrypt("A", None);
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn identity_decrypt_lower_key_upper_input() {
        let s = "HI";
        let iter_in = s.chars();
        let mut iter_out = iter_in.decrypt("a", None);
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn identity_decrypt_upper_key_lower_input() {
        let s = "hi";
        let iter_in = s.chars();
        let mut iter_out = iter_in.decrypt("A", None);
        assert_eq!(iter_out.next(), Some('h'));
        assert_eq!(iter_out.next(), Some('i'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn identity_decrypt_lower_key_lower_input() {
        let s = "hi";
        let iter_in = s.chars();
        let mut iter_out = iter_in.decrypt("a", None);
        assert_eq!(iter_out.next(), Some('h'));
        assert_eq!(iter_out.next(), Some('i'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn minimal_decrypt_upper_key_upper_input() {
        let s = "IJ";
        let iter_in = s.chars();
        let mut iter_out = iter_in.decrypt("B", None);
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn minimal_decrypt_lower_key_upper_input() {
        let s = "IJ";
        let iter_in = s.chars();
        let mut iter_out = iter_in.decrypt("b", None);
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn minimal_decrypt_upper_key_lower_input() {
        let s = "ij";
        let iter_in = s.chars();
        let mut iter_out = iter_in.decrypt("B", None);
        assert_eq!(iter_out.next(), Some('h'));
        assert_eq!(iter_out.next(), Some('i'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn minimal_decrypt_lower_key_lower_input() {
        let s = "ij";
        let iter_in = s.chars();
        let mut iter_out = iter_in.decrypt("b", None);
        assert_eq!(iter_out.next(), Some('h'));
        assert_eq!(iter_out.next(), Some('i'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn bigger_decrypt() {
        let s = "HjJi";
        let iter_in = s.chars();
        let mut iter_out = iter_in.decrypt("ABC", None);
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
        let mut iter_out = iter_in.decrypt("ABC", Some(ForceCase::ToLower));
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
        let mut iter_out = iter_in.decrypt("ABC", Some(ForceCase::ToUpper));
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), None);
    }
}
