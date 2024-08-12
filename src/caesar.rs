use std::collections::HashSet;
use std::convert::TryInto;

pub struct Caesar {
    alphabet: String,
    alphalen: u32,
    offset: u32,
}

impl Caesar {
    pub fn new(alphabet: &'static str, offset: u32) -> Result<Self, String> {
        let char_set: HashSet<char> = alphabet.chars().collect();
        if char_set.len() != alphabet.chars().count() {
            println!("{}", alphabet.len());
            return Err(String::from("Duplicate characters found in alphabet!"));
        }

        let len = <usize as TryInto<u32>>::try_into(alphabet.len()).unwrap();
        Ok(Caesar {
            alphabet: alphabet.to_string(),
            alphalen: len,
            offset: offset % len,
        })
    }
    fn substitute(&self, input: &str, offset: u32) -> String {
        let mut output = String::new();
        let alphabet_chars: Vec<char> = self.alphabet.chars().collect();

        for c in input.chars() {
            if let Some(pos) = alphabet_chars.iter().position(|&x| x == c) {
                let new_pos = (pos as u32 + offset) % self.alphalen;
                output.push(alphabet_chars[new_pos as usize]);
            } else {
                output.push(c);
            }
        }
        output
    }
    pub fn encrypt(&self, plaintext: &str) -> String {
        self.substitute(plaintext, self.offset)
    }
    pub fn decrypt(&self, ciphertext: &str) -> String {
        self.substitute(ciphertext, self.alphalen - self.offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    #[test]
    fn stasis() {
        let plaintext = "I CAME, I SAW, I CONQUERED.";
        let caesar = Caesar::new(ALPHABET, 0).unwrap();
        assert_eq!(caesar.encrypt(plaintext), plaintext);
        assert_eq!(caesar.decrypt(plaintext), plaintext);
    }
    #[test]
    fn shift2() {
        let plaintext = "THE die IS CAST."; // case-sensitive!
        let ciphertxt = "VJG die KU ECUV.";
        let caesar = Caesar::new("ABCDEFGHIJKLMNOPQRSTUVXYZ", 2).unwrap();
        assert_eq!(caesar.encrypt(plaintext), ciphertxt);
        assert_eq!(caesar.decrypt(ciphertxt), plaintext);
    }
    #[test]
    fn wrap27() {
        let plaintext = "ET TU, brute?"; // case-sensitive!
        let ciphertxt = "FU UV, brute?";
        let caesar = Caesar::new(ALPHABET, 27).unwrap();
        assert_eq!(caesar.encrypt(plaintext), ciphertxt);
        assert_eq!(caesar.decrypt(ciphertxt), plaintext);
    }
}
