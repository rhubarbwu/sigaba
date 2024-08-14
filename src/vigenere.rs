use crate::common::{check_unique, Cipher};

#[derive(Debug)]
pub struct Vigenere {
    alphabet: String,
    keystream: String,
    beaufort: bool,
}
impl Vigenere {
    pub fn new(alphabet: &str, keystream: &str) -> Result<Self, String> {
        check_unique(alphabet).unwrap();
        if !keystream.chars().all(|c| alphabet.contains(c)) {
            return Err(String::from("Keyword contains invalid characters!"));
        }
        Ok(Vigenere {
            alphabet: alphabet.to_string(),
            keystream: keystream.to_string(),
            beaufort: false,
        })
    }
    pub fn new_beaufort(alphabet: &str, keystream: &str) -> Result<Self, String> {
        Ok(Self {
            beaufort: true,
            ..Self::new(alphabet, keystream).unwrap()
        })
    }

    fn get_index(&self, c: char) -> Option<usize> {
        self.alphabet.chars().position(|x| x == c)
    }
    fn substitute(&self, text: &str, decrypt: bool) -> String {
        let (mut kw_idx, kw_len) = (0, self.keystream.len());
        let mut result = String::new();

        for c in text.chars() {
            if let Some(txt_idx) = self.get_index(c) {
                let kw_char = self.keystream.chars().nth(kw_idx).unwrap();
                let (ab_len, ab_idx) = (self.alphabet.len(), self.get_index(kw_char).unwrap());

                let new_idx = match (self.beaufort, decrypt) {
                    (true, _) => (ab_len + ab_idx - txt_idx) % ab_len,
                    (_, false) => (txt_idx + ab_idx) % ab_len,
                    (_, true) => (txt_idx + ab_len - ab_idx) % ab_len,
                };
                result.push(self.alphabet.chars().nth(new_idx).unwrap());

                kw_idx = (kw_idx + 1) % kw_len;
            } else {
                result.push(c);
            }
        }
        result
    }
}
impl Cipher for Vigenere {
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
    use crate::common::{ENGLISH, KRYPTOS};

    #[test]
    fn kryptos_k1() {
        let plaintext = "BETWEENSUBTLESHADINGANDTHEABSENCEOFLIGHT";
        let ciphertxt = "EMUFPHZLRFAXYUSDJKZLDKRNSHGNFIVJYQTQUXQB";
        let vigenere = Vigenere::new(&KRYPTOS, "PALIMPSEST").unwrap();
        assert_eq!(vigenere.encrypt(plaintext), ciphertxt);
        assert_eq!(vigenere.decrypt(ciphertxt), plaintext);
    }

    #[test]
    fn kryptos_k2() {
        let plaintext = "IT WAS TOTALLY INVISIBLE HOWS THAT POSSIBLE ?";
        let ciphertxt = "VFPJUDEEHZWETZYVGWHKKQETGFQJNCEGGWHKK?";
        let vigenere = Vigenere::new(&KRYPTOS, "ABSCISSA").unwrap();
        let encrypted = vigenere.encrypt(plaintext);
        assert_eq!(encrypted.replace(" ", ""), ciphertxt);
        let decrypted = vigenere.decrypt(ciphertxt);
        assert_eq!(decrypted, plaintext.replace(" ", ""));
    }

    #[test]
    fn variant() {
        let plaintext = "IT WAS TOTALLY INVISIBLE HOWS THAT POSSIBLE ?";
        let ciphertxt = "BVLXLXZWKCENKGJBZAYBVBZJZVBXIWZZZAYBV?";
        let vigenere = Vigenere::new(&KRYPTOS, "ABSCISSA").unwrap();
        let encrypted = vigenere.decrypt(plaintext);
        assert_eq!(encrypted.replace(" ", ""), ciphertxt);
        let decrypted = vigenere.encrypt(ciphertxt);
        assert_eq!(decrypted.replace(" ", ""), plaintext.replace(" ", ""));
    }

    #[test]
    fn beaufort() {
        let plaintext = "IT WAS GIOVANNI VESTRI";
        let ciphertxt = "XYENKCKRWAAPAXBZHWU";
        let beaufort = Vigenere::new_beaufort(&ENGLISH, "FRANCIS").unwrap();
        let encrypted = beaufort.encrypt(plaintext);
        assert_eq!(encrypted.replace(" ", ""), ciphertxt);
        let decrypted = beaufort.decrypt(ciphertxt);
        assert_eq!(decrypted, plaintext.replace(" ", ""));
    }
}
