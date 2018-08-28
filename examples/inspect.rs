extern crate content_inspector;

use std::env;
use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;
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

        let file = File::open(&filename)?;
        let mut buffer: Vec<u8> = vec![];

        file.take(MAX_PEEK_SIZE as u64).read_to_end(&mut buffer)?;

        let content_type = content_inspector::inspect(&buffer);
        println!("{}: {}", filename, content_type);
    }

    Ok(())
}
