use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{Read};
use std::process;
use std::string::String;
use std::str;

mod config;

type ZCatResult<T> = Result<T, Box<dyn Error>>;




fn print_file(file_path: &String) -> Result<(), Box<dyn Error>> {
    let fp = std::path::Path::new(file_path);
    if !fp.exists() {
        return Err(Box::from(format!("{}: No such file or directory", file_path)));
    }
    let mut f = File::open(fp)?;
    let mut buffer = [0; 4096];

    loop {
        let result = f.read(&mut buffer[..])?;
        if result <= 0 { break }
        match str::from_utf8(&buffer[0..result]) {
            Ok(v) =>  print!("{}", &v[0..result]),
            Err(_) => break,
        };
    }

    return Ok(());
}


fn main() -> ZCatResult<()> {
    let args = config::ZCatArguments::parse();
    let mut failed = false;
    for fp in args.files.iter() {
        match print_file(fp) {
            Err(err) => {
                eprintln!("zcat: {}", err);
                failed = true;
            },
            Ok(_) => {},
        }
    }
    if failed {
        process::exit(1);
    }
    return Ok(()); 
}
