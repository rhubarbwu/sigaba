use clap::{Parser, Subcommand};
use sigaba::affine::Affine;
use sigaba::autokey::AutoKey;
use sigaba::common::Cipher;
use sigaba::common::ENGLISH;
use sigaba::transpose::Transpose;
use sigaba::vigenere::Vigenere;
use std::fs;
use std::process::exit;

#[derive(Debug, Clone, Subcommand)]
enum CipherT {
    Affine {
        #[arg(short, long)]
        factor: isize,
        #[arg(short, long)]
        offset: isize,
    },
    Atbash,
    AutoKey {
        #[arg(short = 'k', long)]
        key: String,
        #[arg(short = 'r', long, default_value_t = false)]
        auto: bool,
    },
    Beaufort {
        #[arg(short = 'k', long)]
        key: String,
    },
    Caesar {
        #[arg(short = 'b', long)]
        shift: isize,
    },
    Rot13,
    Rotate {
        #[arg(short = 'n', long)]
        n_rows: usize,
        #[arg(short = 'p', long, default_value_t = false)]
        pad_cols: bool,
        #[arg(short = 'c', long, default_value_t = false)]
        counter: bool,
    },
    Transpose {
        #[arg(short = 'n', long)]
        n_rows: usize,
        #[arg(short = 'p', long, default_value_t = false)]
        pad_cols: bool,
    },
    Vigenere {
        #[arg(short = 'k', long)]
        key: String,
    },
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short = 'a', long, global = true, default_value_t = ENGLISH.to_string())]
    alphabet: String,

    #[arg(short = 'd', long, global = true)]
    decrypt: bool,

    #[arg(short = 'i', long, global = true, default_value_t = String::new())]
    input: String,

    #[arg(short = 'I', long, global = true, default_value_t = String::new())]
    input_file: String,

    #[arg(short = 'O', long, global = true, default_value_t = String::new())]
    output_file: String,

    #[command(subcommand)]
    cipher: CipherT,
}

fn main() {
    let args = Args::parse();

    let alphabet = &args.alphabet;

    let cipher: &dyn Cipher = match &args.cipher {
        CipherT::Affine { factor, offset } => &Affine::new(alphabet, *factor, *offset).unwrap(),
        CipherT::Atbash => &Affine::new_atbash(alphabet).unwrap(),
        CipherT::AutoKey { key, auto } => &AutoKey::new(alphabet, key, *auto).unwrap(),
        CipherT::Beaufort { key } => &Vigenere::new_beaufort(alphabet, &key).unwrap(),
        CipherT::Caesar { shift } => &Affine::new_caesar(alphabet, *shift).unwrap(),
        CipherT::Rot13 => &Affine::new_rot13().unwrap(),
        CipherT::Transpose { n_rows, pad_cols } => {
            &Transpose::as_flip(alphabet, *n_rows, *pad_cols).unwrap()
        }
        CipherT::Rotate {
            n_rows,
            pad_cols,
            counter,
        } => &(match *counter {
            false => Transpose::as_right,
            true => Transpose::as_left,
        })(alphabet, *n_rows, *pad_cols)
        .unwrap(),
        CipherT::Vigenere { key } => &Vigenere::new(alphabet, &key).unwrap(),
    };

    let input = match fs::read_to_string(&args.input_file) {
        Ok(file_content) => {
            if !args.input.is_empty() {
                println!("W: Both file and direct input provided. The file content will be used.");
            }
            file_content
        }
        Err(_) if !args.input.is_empty() => args.input.clone(),
        Err(_) => {
            println!("E: No input provided.");
            exit(1);
        }
    };

    let output = match args.decrypt {
        false => cipher.encrypt(&input),
        true => cipher.decrypt(&input),
    };

    if args.output_file.len() > 0 {
        if let Err(e) = fs::write(&args.output_file, output) {
            eprintln!("E: failed to write to file: {}", e);
        } else {
            println!("File written successfully: {}", args.output_file);
        }
    } else {
        println!("{}", output);
    }
}
