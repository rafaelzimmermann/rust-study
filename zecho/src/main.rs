use clap::{Parser};

#[derive(Parser, Debug)]
#[command(name = "zecho")]
#[command(author = "Rafael Zimmermann <rafael@blackcoffee.page>")]
#[command(version = "1.0")]
#[command(about = "Display a line of text", long_about = None)]
#[command(trailing_var_arg=true)]
struct Arg {
    #[arg(short, default_value_t = false)]
    strip: bool,
    #[arg(allow_hyphen_values=true)]
    text: Vec<String>,
}

fn main() {
    let cli = Arg::parse();
    let output = format!("{}", cli.text.join(" "));
    println!("{}", output);
}
