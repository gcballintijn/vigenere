enum VigenereMode {
    Encrypt,
    Decrypt,
}

pub struct VigenereIterator<I>
    where I: Iterator<Item = char>
{
    mode: VigenereMode,
    distances: Vec<u8>,
    index: usize,
    iter: I,
}

impl<I> VigenereIterator<I>
    where I: Iterator<Item = char>
{
    const BASE: u8 = 'A' as u8;

    fn new(mode: VigenereMode, key: &str, iter: I) -> Self {
        let distances = key.chars()
            .map(|ch| ch as u8 - Self::BASE)
            .collect::<Vec<u8>>();
        Self {
            mode,
            distances,
            index: 0,
            iter,
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
                let input = ch as u8 - Self::BASE;
                let output = match self.mode {
                    VigenereMode::Encrypt => (input + distance) % 26,
                    VigenereMode::Decrypt => (26 + input - distance) % 26,
                };
                Some((Self::BASE + output) as char)  
            },
            Some(ch) => Some(ch),
            None => None
        }
    }
}

pub trait Encrypt : Iterator<Item = char> + Sized {
    fn encrypt(self, key: &str) -> VigenereIterator<Self>;
}

pub trait Decrypt : Iterator<Item = char> + Sized {
    fn decrypt(self, key: &str) -> VigenereIterator<Self>;
}

impl<I> Encrypt for I
    where I: Iterator<Item = char>
{
    fn encrypt(self, key: &str) -> VigenereIterator<I> {
        VigenereIterator::new(VigenereMode::Encrypt, key, self)
    }
}

impl<I> Decrypt for I
    where I: Iterator<Item = char>
{
    fn decrypt(self, key: &str) -> VigenereIterator<I> {
        VigenereIterator::new(VigenereMode::Decrypt, key, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_encrypt() {
        let s = "HI";
        let iter_in = s.chars();
        let mut iter_out = iter_in.encrypt("A");
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn minimal_encrypt() {
        let s = "HI";
        let iter_in = s.chars();
        let mut iter_out = iter_in.encrypt("B");
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), Some('J'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn bigger_encrypt() {
        let s = "HIHI";
        let iter_in = s.chars();
        let mut iter_out = iter_in.encrypt("ABC");
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('J'));
        assert_eq!(iter_out.next(), Some('J'));
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn identity_decrypt() {
        let s = "HI";
        let iter_in = s.chars();
        let mut iter_out = iter_in.decrypt("A");
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn minimal_decrypt() {
        let s = "IJ";
        let iter_in = s.chars();
        let mut iter_out = iter_in.decrypt("B");
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), None);
    }

    #[test]
    fn bigger_decrypt() {
        let s = "HJJI";
        let iter_in = s.chars();
        let mut iter_out = iter_in.decrypt("ABC");
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), Some('H'));
        assert_eq!(iter_out.next(), Some('I'));
        assert_eq!(iter_out.next(), None);
    }
}
