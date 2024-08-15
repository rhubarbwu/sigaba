use std::collections::HashSet;

pub const ENGLISH: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const KRYPTOS: &str = "KRYPTOSABCDEFGHIJLMNQUVWXZ";

pub fn filter(input: &str, alphabet: &str) -> String {
    input.chars().filter(|c| alphabet.contains(*c)).collect()
}

pub fn refill(input: &str, template: &str, alphabet: &str) -> String {
    let mut output = String::with_capacity(template.len());
    let (input, mut i) = (input.to_string(), 0);
    for j in 0..template.len() {
        if let Some(c) = template.chars().nth(j) {
            if !alphabet.contains(c) {
                output.push(c);
            } else if let Some(c) = input.chars().nth(i) {
                output.push(c);
                i += 1;
            }
        }
    }
    output
}

pub fn check_unique(alphabet: &str) -> Result<(), String> {
    let char_set: HashSet<char> = alphabet.chars().collect();
    if char_set.len() != alphabet.chars().count() {
        println!("{}", alphabet.len());
        return Err(String::from("Duplicate characters found in alphabet!"));
    }
    Ok(())
}

pub fn mult_inv(a: isize, m: isize) -> Result<isize, String> {
    for i in 0..m {
        if (a * i + m) % m == 1 {
            return Ok(i);
        }
    }
    Err(String::from("Can't compute multiplicative inverse!"))
}

pub trait Cipher {
    fn encrypt(&self, plaintext: &str) -> String;
    fn decrypt(&self, ciphertxt: &str) -> String;
}
