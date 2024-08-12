use std::collections::HashSet;

pub const ENGLISH: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const KRYPTOS: &str = "KRYPTOSABCDEFGHIJLMNQUVWXZ";

pub fn check_unique(alphabet: &'static str) -> Result<(), String> {
    let char_set: HashSet<char> = alphabet.chars().collect();
    if char_set.len() != alphabet.chars().count() {
        println!("{}", alphabet.len());
        return Err(String::from("Duplicate characters found in alphabet!"));
    }
    Ok(())
}

pub trait Cipher {
    fn encrypt(&self, plaintext: &str) -> String;
    fn decrypt(&self, ciphertxt: &str) -> String;
}
