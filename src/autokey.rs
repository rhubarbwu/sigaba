use crate::common::{filter, refill, Cipher};
use crate::vigenere::Vigenere;

#[derive(Debug)]
pub struct AutoKey {
    alphabet: String,
    primer: String,
    autoregressive: bool,
}

impl AutoKey {
    pub fn new(alphabet: &str, primer: &str, autoregressive: bool) -> Result<Self, String> {
        if !primer.chars().all(|c| alphabet.contains(c)) {
            return Err(String::from("Keyword contains invalid characters!"));
        }
        Ok(Self {
            alphabet: alphabet.to_string(),
            primer: primer.to_string(),
            autoregressive,
        })
    }

    fn parallelize(&self, input: &str) -> String {
        let keystream = self.primer.to_owned() + &filter(input, &self.alphabet);
        let vig = Vigenere::new(&self.alphabet, &keystream).unwrap();
        match self.autoregressive {
            false => vig.encrypt(&input),
            true => vig.decrypt(&input),
        }
    }
    fn autoregress(&self, input: &str) -> String {
        let input_filtered = filter(input, &self.alphabet);
        let mut output = String::new();
        let mut key = self.primer.to_string();
        let (input_len, chunk_size) = (input_filtered.len(), self.primer.len());
        for i in (0..input_len).step_by(chunk_size) {
            let chunk = &input_filtered[i..std::cmp::min(i + chunk_size, input_len)];
            let vig = Vigenere::new(&self.alphabet, &key).unwrap();
            let substitute = match self.autoregressive {
                false => vig.decrypt(&chunk),
                true => vig.encrypt(&chunk),
            };
            output.push_str(&substitute);
            key = substitute;
        }
        refill(&output, input, &self.alphabet)
    }
}

impl Cipher for AutoKey {
    fn encrypt(&self, plaintext: &str) -> String {
        match self.autoregressive {
            false => self.parallelize(plaintext),
            true => self.autoregress(plaintext),
        }
    }
    fn decrypt(&self, ciphertxt: &str) -> String {
        match self.autoregressive {
            false => self.autoregress(ciphertxt),
            true => self.parallelize(ciphertxt),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ENGLISH;

    const STRATEGY: &str = "ATTACK AT DAWN";

    #[test]
    fn txtautokey() {
        let ciphertxt = "QNXEPV YT WTWP";
        let autovig = AutoKey::new(&ENGLISH, "QUEENLY", false).unwrap();
        assert_eq!(autovig.encrypt(&STRATEGY), ciphertxt);
        assert_eq!(autovig.decrypt(&ciphertxt), STRATEGY);
    }

    #[test]
    fn keyautokey() {
        let ciphertxt = "QNXEPV YJ QXAC";
        let autovig = AutoKey::new(&ENGLISH, "QUEENLY", true).unwrap();
        assert_eq!(autovig.encrypt(&STRATEGY), ciphertxt);
        assert_eq!(autovig.decrypt(&ciphertxt), STRATEGY);
    }
}
