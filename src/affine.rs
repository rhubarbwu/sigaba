use super::common::{check_unique, mult_inv, Cipher, ENGLISH};

#[derive(Debug)]
pub struct Affine {
    alphabet: String,
    alphalen: i32,
    factor: i32,
    facinv: i32,
    offset: i32,
}

impl Affine {
    pub fn new(alphabet: &str, factor: i32, offset: i32) -> Result<Self, String> {
        check_unique(alphabet).unwrap();
        let len = <usize as TryInto<i32>>::try_into(alphabet.len()).unwrap();

        let (factor, facinv) = match mult_inv(factor, len) {
            Ok(fi) => (factor, fi),
            Err(_) => (1, 1),
        };

        Ok(Affine {
            alphabet: alphabet.to_string(),
            alphalen: len,
            factor,
            facinv,
            offset,
        })
    }
    pub fn new_atbash(alphabet: &str) -> Result<Self, String> {
        Affine::new(alphabet, -1, -1)
    }
    pub fn new_caesar(alphabet: &str, shift: i32) -> Result<Self, String> {
        Affine::new(alphabet, 1, shift)
    }
    pub fn new_rot13() -> Result<Self, String> {
        Affine::new(&ENGLISH, 1, 13)
    }

    fn substitute(&self, text: &str, decrypt: bool) -> String {
        let mut output = String::new();
        let alphabet_chars: Vec<char> = self.alphabet.chars().collect();
        for c in text.chars() {
            if let Some(pos) = alphabet_chars.iter().position(|&x| x == c) {
                let new_pos = match decrypt {
                    false => self.factor * pos as i32 + self.offset + self.alphalen,
                    true => self.facinv * (self.alphalen + pos as i32 - self.offset),
                } % self.alphalen;
                output.push(alphabet_chars[new_pos as usize]);
            } else {
                output.push(c);
            }
        }
        output
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
