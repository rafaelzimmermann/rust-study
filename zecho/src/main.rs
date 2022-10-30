use clap::Command;

fn main() {
    let _matches = Command::new("zecho")
        .version("0.1.0")
        .author("Rafael Zimmermann")
        .about("Rust echo")
        .get_matches();
}
