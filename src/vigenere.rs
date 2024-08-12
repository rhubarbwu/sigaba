use std::collections::HashSet;

pub struct Vigenere {
    alphabet: String,
    keystream: String,
}
impl Vigenere {
    pub fn new(alphabet: &'static str, keystream: &'static str) -> Result<Self, String> {
        let char_set: HashSet<char> = alphabet.chars().collect();
        if char_set.len() != alphabet.chars().count() {
            return Err(String::from("Duplicate characters found in alphabet!"));
        }

        if !keystream.chars().all(|c| char_set.contains(&c)) {
            return Err(String::from("Keyword contains invalid characters!"));
        }

        Ok(Vigenere {
            alphabet: alphabet.to_string(),
            keystream: keystream.to_string(),
        })
    }
    fn get_index(&self, c: char) -> Option<usize> {
        self.alphabet.chars().position(|x| x == c)
    }
    fn substitute(&self, text: &str, decrypt: bool) -> String {
        let kw_length = self.keystream.len();

        let mut result = String::new();
        let mut kw_idx = 0;

        for c in text.chars() {
            if let Some(txt_idx) = self.get_index(c) {
                let kw_char = self.keystream.chars().nth(kw_idx % kw_length).unwrap();
                let ab_len = self.alphabet.len();
                let ab_idx = self.get_index(kw_char).unwrap();

                let offset = if decrypt { ab_len - ab_idx } else { ab_idx };
                let new_idx = (txt_idx + offset) % ab_len;
                result.push(self.alphabet.chars().nth(new_idx).unwrap());

                kw_idx += 1;
            } else {
                result.push(c);
            }
        }
        result
    }
    pub fn encrypt(&self, plaintext: &str) -> String {
        self.substitute(plaintext, false)
    }
    pub fn decrypt(&self, ciphertext: &str) -> String {
        self.substitute(ciphertext, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const KRYPTOS: &str = "KRYPTOSABCDEFGHIJLMNQUVWXZ";

    #[test]
    fn kryptos_k1() {
        let plaintext = "BETWEENSUBTLESHADINGANDTHEABSENCEOFLIGHT";
        let ciphertxt = "EMUFPHZLRFAXYUSDJKZLDKRNSHGNFIVJYQTQUXQB";

        let vigenere = Vigenere::new(KRYPTOS, "PALIMPSEST").unwrap();
        assert_eq!(vigenere.encrypt(plaintext), ciphertxt);
        assert_eq!(vigenere.decrypt(ciphertxt), plaintext);
    }
    #[test]
    fn kryptos_k2() {
        let plaintext = "IT WAS TOTALLY INVISIBLE HOWS THAT POSSIBLE ?";
        let ciphertxt = "VFPJUDEEHZWETZYVGWHKKQETGFQJNCE GGWHKK?";

        let vigenere = Vigenere::new(KRYPTOS, "ABSCISSA").unwrap();
        let encrypted = vigenere.encrypt(plaintext);
        assert_eq!(encrypted.replace(" ", ""), ciphertxt.replace(" ", ""));
        let decrypted = vigenere.decrypt(ciphertxt);
        assert_eq!(decrypted.replace(" ", ""), plaintext.replace(" ", ""));
    }
}
