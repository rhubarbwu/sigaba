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
        Ok(Self {
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

    const K1_PLAIN: &str = "BETWEEN SUBTLE SHADING AND THE ABSENCE OF LIGHT";
    const K2_PLAIN: &str = "IT WAS TOTALLY INVISIBLE HOWS THAT POSSIBLE ?";

    #[test]
    fn kryptos_k1() {
        let ciphertxt = "EMUFPHZ LRFAXY USDJKZL DKR NSH GNFIVJY QT QUXQB";
        let vigenere = Vigenere::new(&KRYPTOS, "PALIMPSEST").unwrap();
        assert_eq!(vigenere.encrypt(K1_PLAIN), ciphertxt);
        assert_eq!(vigenere.decrypt(ciphertxt), K1_PLAIN);
    }

    #[test]
    fn kryptos_k2() {
        let ciphertxt = "VF PJU DEEHZWE TZYVGWHKK QETG FQJN CEGGWHKK ?";
        let vigenere = Vigenere::new(&KRYPTOS, "ABSCISSA").unwrap();
        assert_eq!(vigenere.encrypt(K2_PLAIN), ciphertxt);
        assert_eq!(vigenere.decrypt(ciphertxt), K2_PLAIN);
    }

    #[test]
    fn variant() {
        let ciphertxt = "BV LXL XZWKCEN KGJBZAYBV BZJZ VBXI WZZZAYBV ?";
        let vigenere = Vigenere::new(&KRYPTOS, "ABSCISSA").unwrap();
        assert_eq!(vigenere.decrypt(K2_PLAIN), ciphertxt);
        assert_eq!(vigenere.encrypt(ciphertxt), K2_PLAIN);
    }

    #[test]
    fn beaufort() {
        let plaintext = "IT WAS GIOVANNI VESTRI";
        let ciphertxt = "XYENKCKRWAAPAXBZHWU";
        let beaufort = Vigenere::new_beaufort(&ENGLISH, "FRANCIS").unwrap();
        assert_eq!(beaufort.encrypt(plaintext).replace(" ", ""), ciphertxt);
        assert_eq!(beaufort.decrypt(ciphertxt), plaintext.replace(" ", ""));
    }
}
