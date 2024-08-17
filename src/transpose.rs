use crate::common::Cipher;
use crate::common::{alphabetize, char_index, check_unique, filter, refill};
use rand::Rng;

use ndarray::{s, Array1};

pub fn transpose_vec(elems: Vec<u32>, shape: (usize, usize)) -> Vec<u32> {
    let n_elems = elems.len();
    let matrix = Array1::from_vec(elems)
        .into_shape_with_order(shape)
        .expect("Failed to fold into matrix.");
    let transpose = matrix.t().as_standard_layout().to_owned();
    transpose.into_shape_with_order(n_elems).unwrap().to_vec()
}

pub fn rotate_vec(elems: Vec<u32>, shape: (usize, usize), counter: bool) -> Vec<u32> {
    let slice_info = match counter {
        false => s![.., ..;-1],
        true => s![..;-1, ..],
    };
    let n_elems = elems.len();
    let matrix = Array1::from_vec(elems)
        .into_shape_with_order(shape)
        .expect("Failed to fold into matrix.");
    let rotated = matrix.t().slice(slice_info).as_standard_layout().to_owned();
    rotated.into_shape_with_order(n_elems).unwrap().to_vec()
}

#[derive(Debug)]
enum MatrixOp {
    Transpose,
    RotateRight,
    RotateLeft,
}
#[derive(Debug)]
pub struct Transpose {
    alphabet: String,
    num_rows: usize,
    pad_cols: bool,
    matrixop: MatrixOp,
}
impl Transpose {
    fn new(
        alphabet: &str,
        num_rows: usize,
        pad_cols: bool,
        matrixop: MatrixOp,
    ) -> Result<Self, String> {
        check_unique(alphabet).unwrap();
        Ok(Self {
            alphabet: alphabet.to_string(),
            num_rows,
            pad_cols,
            matrixop,
        })
    }
    pub fn as_flip(alphabet: &str, num_rows: usize, pad_cols: bool) -> Result<Self, String> {
        Self::new(alphabet, num_rows, pad_cols, MatrixOp::Transpose)
    }
    pub fn as_right(alphabet: &str, num_rows: usize, pad_cols: bool) -> Result<Self, String> {
        Self::new(alphabet, num_rows, pad_cols, MatrixOp::RotateRight)
    }
    pub fn as_left(alphabet: &str, num_rows: usize, pad_cols: bool) -> Result<Self, String> {
        Self::new(alphabet, num_rows, pad_cols, MatrixOp::RotateLeft)
    }

    fn transpose(&self, input: &str, decrypt: bool) -> String {
        let clean = filter(input, &self.alphabet);
        let mut n_rows = self.num_rows;
        let mut n_cols = (clean.len() + self.num_rows - 1) / self.num_rows;
        if decrypt {
            (n_cols, n_rows) = (self.num_rows, n_cols)
        }
        let (n_pad, v_pad) = (n_rows * n_cols - clean.len(), self.alphabet.len() as u32);

        let mut chars = char_index(&clean, &self.alphabet);
        if self.pad_cols != decrypt {
            let mut idx = n_cols - 1;
            for row in 0..n_rows {
                if row >= (n_rows - n_pad) {
                    chars.insert(idx, v_pad)
                }
                idx += n_cols;
            }
        } else {
            chars.resize(chars.len() + n_pad, v_pad);
        }

        let matrixed = match self.matrixop {
            MatrixOp::Transpose => transpose_vec(chars, (n_rows, n_cols)),
            MatrixOp::RotateRight => rotate_vec(chars, (n_rows, n_cols), false),
            MatrixOp::RotateLeft => rotate_vec(chars, (n_rows, n_cols), true),
        };
        let output = alphabetize(matrixed.to_vec(), &self.alphabet);
        refill(&output, input, &self.alphabet)
    }
}
impl Cipher for Transpose {
    fn encrypt(&self, plaintext: &str) -> String {
        self.transpose(plaintext, false)
    }
    fn decrypt(&self, ciphertxt: &str) -> String {
        self.transpose(ciphertxt, true)
    }
}

#[derive(Debug)]
pub struct Columnar {
    alphabet: String,
    keyword: String,
}
impl Columnar {
    pub fn new(alphabet: &str, keyword: &str) -> Result<Self, String> {
        check_unique(alphabet).unwrap();
        Ok(Self {
            alphabet: alphabet.to_string(),
            keyword: keyword.to_string(),
        })
    }
}
impl Cipher for Columnar {
    fn encrypt(&self, plaintext: &str) -> String {
        let clean = filter(plaintext, &self.alphabet);
        let n_cols = self.keyword.len();
        let n_rows = (clean.len() + n_cols - 1) / n_cols;
        let n_pad = n_cols * n_rows - clean.len();

        let mut rng = rand::thread_rng();
        let rand_pad: Vec<u32> = (0..n_pad)
            .map(|_| rng.gen_range(0..self.alphabet.len() as u32))
            .collect();
        let chars = [
            char_index(&self.keyword, &self.alphabet),
            char_index(&clean, &self.alphabet),
            rand_pad,
        ]
        .concat();

        let mut cols: Vec<Vec<u32>> = vec![Vec::with_capacity(n_rows); n_cols];
        for (i, &value) in chars.iter().enumerate() {
            cols[i % n_cols].push(value);
        }
        cols.sort_by_key(|v| v[0]);

        let joined = cols
            .into_iter()
            .map(|col| col.into_iter().skip(1))
            .flatten()
            .collect();
        let output = alphabetize(joined, &self.alphabet);
        refill(&output, plaintext, &self.alphabet)
    }

    fn decrypt(&self, ciphertxt: &str) -> String {
        let clean = filter(ciphertxt, &self.alphabet);
        let n_cols = self.keyword.len();
        let n_rows = (clean.len() + n_cols - 1) / n_cols;

        let kw_idx = char_index(&self.keyword, &self.alphabet);
        let mut kw_idx_to_sort = kw_idx.clone();
        kw_idx_to_sort.sort_unstable();

        let chars = [char_index(&clean, &self.alphabet)].concat();

        let mut cols: Vec<Vec<u32>> = vec![Vec::with_capacity(n_rows); n_cols];
        let mut idx = 0;
        for col in 0..n_cols {
            cols[col].push(kw_idx_to_sort[col]);
            for _ in 0..n_rows {
                cols[col].push(chars[idx]);
                idx += 1;
            }
        }
        cols.sort_by_key(|v| kw_idx.iter().position(|&x| x == v[0]));

        let joined = cols
            .into_iter()
            .map(|col| col.into_iter().skip(1))
            .flatten()
            .collect();
        let transpose = transpose_vec(joined, (n_cols, n_rows));
        let output = alphabetize(transpose, &self.alphabet);
        refill(&output, ciphertxt, &self.alphabet)
    }
}

pub struct RailFence {}
impl RailFence {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ENGLISH;

    const PLAINTEXT: &str = "WE ARE DISCOVERED FLEE AT ONCE";

    #[test]
    fn scytale_symm() {
        let plaintext = PLAINTEXT.replace("AT ONCE", "QUICKLY");
        let ciphertxt = "WO EEV QAEURRIEEC DDKI FLSLYCE";

        let pad_rows = Transpose::as_flip(&ENGLISH, 3, false).unwrap();
        assert_eq!(pad_rows.encrypt(&plaintext), ciphertxt);
        assert_eq!(pad_rows.decrypt(ciphertxt), plaintext);

        let pad_cols = Transpose::as_flip(&ENGLISH, 3, true).unwrap();
        assert_eq!(pad_cols.encrypt(&plaintext), ciphertxt);
        assert_eq!(pad_cols.decrypt(ciphertxt), plaintext);
    }

    #[test]
    fn scytale_asym() {
        let ciphertxt = "WO EEV AAETRROEEN DDCI FE SLCE";
        let pad_rows = Transpose::as_flip(&ENGLISH, 3, false).unwrap();
        assert_eq!(pad_rows.encrypt(PLAINTEXT), ciphertxt);
        assert_eq!(pad_rows.decrypt(&ciphertxt), PLAINTEXT);

        let ciphertxt = "WO EEV EAEARRTEEO DDNI FC SLEC";
        let pad_cols = Transpose::as_flip(&ENGLISH, 3, true).unwrap();
        assert_eq!(pad_cols.encrypt(PLAINTEXT), ciphertxt);
        assert_eq!(pad_cols.decrypt(&ciphertxt), PLAINTEXT);
    }

    #[test]
    fn columnar() {
        let ciphertxt = "EV LN* ACDT*ESEA* ROFO *D EEC*WIREE";
        let pad_rows = Columnar::new(&ENGLISH, "ZEBRAS").unwrap();
        let encrypted = pad_rows.encrypt(PLAINTEXT);
        let cleaned: String = ciphertxt
            .chars()
            .zip(encrypted.chars())
            .filter_map(|(c0, c1)| if c0 == '*' { None } else { Some(c1) })
            .collect();
        assert_eq!(cleaned, ciphertxt.replace("*", ""));
        assert_eq!(pad_rows.decrypt(&encrypted)[..PLAINTEXT.len()], *PLAINTEXT);
    }

    #[test]
    fn rotate() {
        assert_eq!("", "ROTATION NEEDS TESTING");
    }
}
