use std::collections::HashSet;

pub const ENGLISH: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const KRYPTOS: &str = "KRYPTOSABCDEFGHIJLMNQUVWXZ";

pub fn check_unique(alphabet: &str) -> Result<(), String> {
    let char_set: HashSet<char> = alphabet.chars().collect();
    if char_set.len() != alphabet.chars().count() {
        println!("{}", alphabet.len());
        return Err(String::from("Duplicate characters found in alphabet!"));
    }
    Ok(())
}

pub fn mult_inv(a: i32, m: i32) -> Result<i32, String> {
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
