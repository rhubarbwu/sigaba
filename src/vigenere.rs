use crate::common::{check_unique, filter, refill, Cipher};

#[derive(Debug)]
pub struct Vigenere {
    alphabet: String,
    alphalen: usize,
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
            alphalen: alphabet.len(),
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

    fn substitute(&self, input: &str, decrypt: bool) -> String {
        let mut output = String::with_capacity(input.len());
        let (mut kw_idx, kw_len) = (0, self.keystream.len());
        for c in filter(input, &self.alphabet).chars() {
            let txt_idx = self.alphabet.chars().position(|x| x == c).unwrap();
            let kw_char = self.keystream.chars().nth(kw_idx).unwrap();
            let ab_idx = self.alphabet.chars().position(|x| x == kw_char).unwrap();
            let new_idx = match (self.beaufort, decrypt) {
                (true, _) => (self.alphalen + ab_idx - txt_idx) % self.alphalen,
                (_, false) => (txt_idx + ab_idx) % self.alphalen,
                (_, true) => (txt_idx + self.alphalen - ab_idx) % self.alphalen,
            };
            output.push(self.alphabet.chars().nth(new_idx).unwrap());
            kw_idx = (kw_idx + 1) % kw_len;
        }
        refill(&output, input, &self.alphabet)
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
