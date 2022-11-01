use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "zcat")]
#[command(author = "Rafael Zimmermann <rafael@blackcoffee.page>")]
#[command(version = "0.1.0")]
#[command(about = "Display a line of text", long_about = None)]
pub struct ZCatArguments {
    #[arg(short('n'), long, default_value_t = false, help = "number all output lines")]
    pub number: bool,
    #[arg(short('b'), long, default_value_t = false, help = "number nonempty output lines, overrides -n")]
    pub number_non_blank_lines: bool,
    #[arg(allow_hyphen_values=true)]
    pub files: Vec<String>,
}