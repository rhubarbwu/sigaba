use super::common::{check_unique, filter, mult_inv, refill, Cipher, ENGLISH};

#[derive(Debug)]
pub struct Affine {
    alphabet: String,
    alphalen: usize,
    factor: isize,
    facinv: isize,
    offset: isize,
}

impl Affine {
    pub fn new(alphabet: &str, factor: isize, offset: isize) -> Result<Self, String> {
        check_unique(alphabet).unwrap();
        let len = <usize as TryInto<isize>>::try_into(alphabet.len()).unwrap();
        let (factor, facinv) = match mult_inv(factor, len) {
            Ok(fi) => (factor, fi),
            Err(_) => (1, 1),
        };

        Ok(Self {
            alphabet: alphabet.to_string(),
            alphalen: alphabet.len(),
            factor,
            facinv,
            offset,
        })
    }
    pub fn new_atbash(alphabet: &str) -> Result<Self, String> {
        Self::new(alphabet, -1, -1)
    }
    pub fn new_caesar(alphabet: &str, shift: isize) -> Result<Self, String> {
        Self::new(alphabet, 1, shift)
    }
    pub fn new_rot13() -> Result<Self, String> {
        Self::new(&ENGLISH, 1, 13)
    }

    fn substitute(&self, input: &str, decrypt: bool) -> String {
        let mut output = String::with_capacity(input.len());
        for c in filter(input, &self.alphabet).chars() {
            let idx = self.alphabet.chars().position(|x| x == c).unwrap();
            let new_idx = match decrypt {
                false => self.factor * idx as isize + self.offset + self.alphalen as isize,
                true => self.facinv * (self.alphalen as isize + idx as isize - self.offset),
            } % self.alphalen as isize;
            output.push(self.alphabet.chars().nth(new_idx as usize).unwrap());
        }
        refill(&output, input, &self.alphabet)
    }
}
impl Cipher for Affine {
    fn encrypt(&self, plaintext: &str) -> String {
        self.substitute(plaintext, false)
    }
    fn decrypt(&self, ciphertxt: &str) -> String {
        self.substitute(ciphertxt, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity() {
        let plaintext = "DIFFERENT DAY, SAME OLD NONSENSE";
        let iden = Affine::new(&ENGLISH, 1, 0).unwrap();
        assert_eq!(iden.encrypt(&plaintext), plaintext);
        assert_eq!(iden.decrypt(&plaintext), plaintext);
    }

    #[test]
    fn rotate13() {
        let plaintext = "THIS IS A SPOILER";
        let ciphertxt = "GUVF VF N FCBVYRE";
        let rot13 = Affine::new_rot13().unwrap();
        assert_eq!(rot13.encrypt(&plaintext), ciphertxt);
        assert_eq!(rot13.decrypt(&ciphertxt), plaintext);
    }

    #[test]
    fn caesar12() {
        let plaintext = "THE die IS CAST."; // case-sensitive!
        let ciphertxt = "FTQ die UE OMEF.";
        let caesar = Affine::new_caesar(&ENGLISH, 12).unwrap();
        assert_eq!(caesar.encrypt(&plaintext), ciphertxt);
        assert_eq!(caesar.decrypt(&ciphertxt), plaintext);
    }

    #[test]
    fn caesar27() {
        let plaintext = "ET TU, brute?"; // case-sensitive!
        let ciphertxt = "FU UV, brute?";
        let caesar = Affine::new_caesar(&ENGLISH, 27).unwrap();
        assert_eq!(caesar.encrypt(&plaintext), ciphertxt);
        assert_eq!(caesar.decrypt(&ciphertxt), plaintext);
    }

    #[test]
    fn atbash() {
        let plaintext = "PALINDROME";
        let ciphertxt = "KZORMWILNV";
        let atbash = Affine::new_atbash(&ENGLISH).unwrap();
        assert_eq!(atbash.encrypt(&plaintext), ciphertxt);
        assert_eq!(atbash.decrypt(&ciphertxt), plaintext);
    }

    #[test]
    fn affine() {
        let plaintext = "MICHAEL"; // case-sensitive!
        let ciphertxt = "PZBITJY";
        let affine = Affine::new(&ENGLISH, 17, 19).unwrap();
        assert_eq!(affine.encrypt(&plaintext), ciphertxt);
        assert_eq!(affine.decrypt(&ciphertxt), plaintext);
    }
}
