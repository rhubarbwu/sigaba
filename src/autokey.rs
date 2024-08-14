use crate::common::Cipher;
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
        Ok(AutoKey {
            alphabet: alphabet.to_string(),
            primer: primer.to_string(),
            autoregressive,
        })
    }

    fn parallelize(&self, input: &str) -> String {
        let filtered: String = input
            .chars()
            .filter(|c| self.alphabet.contains(*c))
            .collect();
        let keystream = self.primer.to_owned() + &filtered;
        let vig = Vigenere::new(&self.alphabet, &keystream).unwrap();
        match self.autoregressive {
            false => vig.encrypt(&input),
            true => vig.decrypt(&input),
        }
    }

    fn autoregress(&self, input: &str) -> String {
        let mut key = self.primer.to_string();
        let mut output = String::new();
        let (input_len, chunk_size) = (input.len(), self.primer.len());
        for i in (0..input_len).step_by(chunk_size) {
            let chunk = &input[i..std::cmp::min(i + chunk_size, input_len)];
            if !chunk.chars().any(|c| self.alphabet.contains(c)) {
                output.push_str(&chunk);
                continue;
            }

            let vig = Vigenere::new(&self.alphabet, &key).unwrap();
            let substitute = match self.autoregressive {
                false => vig.decrypt(&chunk),
                true => vig.encrypt(&chunk),
            };
            output.push_str(&substitute);

            let filtered: String = substitute
                .chars()
                .filter(|c| self.alphabet.contains(*c))
                .collect();
            key.push_str(&filtered);
            key.drain(..filtered.len());
        }
        output
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

    #[test]
    fn txtautokey() {
        let plaintext = "ATTACK AT DAWN";
        let ciphertxt = "QNXEPV YT WTWP";
        let autovig = AutoKey::new(&ENGLISH, "QUEENLY", false).unwrap();
        assert_eq!(autovig.encrypt(&plaintext), ciphertxt);
        assert_eq!(autovig.decrypt(&ciphertxt), plaintext);
    }

    #[test]
    fn keyautokey() {
        let plaintext = "ATTACK AT DAWN";
        let ciphertxt = "QNXEPV YJ QXAC";
        let autovig = AutoKey::new(&ENGLISH, "QUEENLY", true).unwrap();
        assert_eq!(autovig.encrypt(&plaintext), ciphertxt);
        assert_eq!(autovig.decrypt(&ciphertxt), plaintext);
    }
}
