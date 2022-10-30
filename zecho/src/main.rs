use clap::Parser;
use std::string::String;

#[derive(Parser, Debug)]
#[command(name = "zecho")]
#[command(author = "Rafael Zimmermann <rafael@blackcoffee.page>")]
#[command(version = "1.0")]
#[command(about = "Display a line of text", long_about = None)]
#[command(trailing_var_arg=true)]
struct Arg {
    #[arg(short, default_value_t = false, help = "remove trailling spaces")]
    strip: bool,
    #[arg(short, default_value_t = false, help = "do not output the trailing newline")]
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

    if cli.strip {
        text = remove_whitespace(&text);
    }
    let output = format!("{}", text);
    let sufix = if cli.no_new_line { "" } else { "\n" };
    println!("{}{}", output, sufix);
}
