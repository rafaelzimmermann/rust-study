use clap::Parser;
use std::string::String;

const EXIT_CODE_ARG_ERROR: i32 = 1;

#[derive(Parser, Debug)]
#[command(name = "zecho")]
#[command(author = "Rafael Zimmermann <rafael@blackcoffee.page>")]
#[command(version = "0.1.0")]
#[command(about = "Display a line of text", long_about = None)]
#[command(trailing_var_arg=true)]
struct Arg {
    #[arg(short, default_value_t = false, help = "remove trailling spaces")]
    strip: bool,
    #[arg(short, default_value_t = true, help = "do not output the trailing newline")]
    no_new_line: bool,
    #[arg(allow_hyphen_values=true)]
    text: Vec<String>,
}

fn remove_whitespace(s: &String) -> String{
    return s.split_whitespace().collect();
}

fn main() {
    let cli = Arg::parse();
    let mut text =  cli.text.join(" ");
    if text.is_empty() {
        eprintln!("No text provided");
        std::process::exit(EXIT_CODE_ARG_ERROR);
    }

    text = text.replace(r"\n$", "");

    if cli.strip {
        text = remove_whitespace(&text);
    }
    let output = format!("{}", text);
    let sufix = if cli.no_new_line { "" } else { "\n" };
    println!("{}{}", output, sufix);
}
