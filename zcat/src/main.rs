use clap::Parser;
use freader::BufferedFileReader;
use std::error::Error;
use std::process;
use std::string::String;

mod config;
mod freader;

type ZCatResult<T> = Result<T, Box<dyn Error>>;


fn number_lines(input: String, mut counter: i32) -> Result<String, Box<dyn Error>> {
    let result = input.split('\n').map(|v| {
        counter += 1;
        return format!("{} {}", counter, v);
    }).collect::<Vec<String>>().join("\n");
    return Ok(result);
}

fn number_non_empty_lines(input: String, mut counter: i32) -> Result<String, Box<dyn Error>> {
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


fn format_buffer(value: String, counter: i32, config: &config::ZCatArguments) -> Result<String, Box<dyn Error>> {
    if config.number {
        return number_lines(value, counter);
    } else if config.number_non_blank_lines {
        return number_non_empty_lines(value, counter);
    } 
    return Ok(value.to_string());
}

fn print_file(file_path: &String, config: &config::ZCatArguments) -> Result<(), Box<dyn Error>> {
    let fp = std::path::Path::new(file_path);

    let line_counter = 0;
    let reader: BufferedFileReader<4028> = BufferedFileReader::new(fp)?;
    reader.for_each(|buffer_content: String| {
        match  format_buffer(buffer_content, line_counter, config) {
            Ok(v) => print!("{}", v),
            Err(_) => {},
        }
        
    });

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
