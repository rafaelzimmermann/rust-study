use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{Read};
use std::process;
use std::string::String;
use std::str;

mod config;

type ZCatResult<T> = Result<T, Box<dyn Error>>;


fn number_lines(input: &str, mut counter: i32) -> Result<String, Box<dyn Error>> {
    let result = input.split('\n').map(|v| {
        counter += 1;
        return format!("{} {}", counter, v);
    }).collect::<Vec<String>>().join("\n");
    return Ok(result);
}

fn number_non_empty_lines(input: &str, mut counter: i32) -> Result<String, Box<dyn Error>> {
    let result = input.split('\n').map(|v| {
        if v.is_empty() {
            return format!("{}", v);    
        } else {
            counter += 1;
            return format!("{} {}", counter, v);
        }
    }).collect::<Vec<String>>().join("\n");
    return Ok(result);
}


fn format_buffer(value: &str, counter: i32, config: &config::ZCatArguments) -> Result<String, Box<dyn Error>> {
    if config.number {
        return number_lines(value, counter);
    } else if config.number_non_blank_lines {
        return number_non_empty_lines(value, counter);
    } 
    return Ok(value.to_string());
}

fn print_file(file_path: &String, config: &config::ZCatArguments) -> Result<(), Box<dyn Error>> {
    let fp = std::path::Path::new(file_path);
    if !fp.exists() {
        return Err(Box::from(format!("{}: No such file or directory", file_path)));
    }
    let mut f = File::open(fp)?;
    let mut buffer = [0; 4096];
    let line_counter = 0;
    loop {
        let result = f.read(&mut buffer[..])?;
        if result <= 0 { break }
        match str::from_utf8(&buffer[0..result]) {
            Ok(v) => {
                print!("{}", format_buffer(v, line_counter, config)?);
            },
            Err(_) => break,
        };
    }

    return Ok(());
}


fn main() -> ZCatResult<()> {
    let args = config::ZCatArguments::parse();
    let mut failed = false;
    for fp in args.files.iter() {
        match print_file(fp, &args) {
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
