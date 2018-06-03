extern crate content_inspector;

use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{Error, Read};
use std::process::exit;

const MAX_PEEK_SIZE: usize = 1024;

fn main() -> Result<(), Error> {
    let mut args = env::args();

    if args.len() < 2 {
        eprintln!("USAGE: inspect FILE [FILE...]");
        exit(1);
    }

    args.next();

    for filename in args {
        if !Path::new(&filename).is_file() {
            continue;
        }

        let mut file = File::open(&filename)?;
        let mut buffer = [0; MAX_PEEK_SIZE];

        let length = file.read(&mut buffer[..])?;

        let content_type = content_inspector::inspect(&buffer[0..length]);
        println!("{}: {}", filename, content_type);
    }

    Ok(())
}
