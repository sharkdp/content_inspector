extern crate text_or_binary;

use std::env;
use std::fs::File;
use std::io::{Error, Read};
use std::process::exit;

use text_or_binary::is_text;

const MAX_NUM_BYTES: usize = 1024;

fn main() -> Result<(), Error> {
    if let Some(filename) = env::args().nth(1) {
        let mut file = File::open(&filename)?;
        let mut buffer = [0; MAX_NUM_BYTES];

        let length = file.read(&mut buffer[..])?;

        if is_text(&buffer[0..length]) {
            println!("{} contains printable text", filename);
            exit(0);
        }

        println!("{} contains binary data", filename);
        exit(1);
    } else {
        eprintln!("USAGE: is_printable <FILE>");
        exit(1);
    }
}
