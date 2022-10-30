fn main() {
    // cargo run -- your args here
    // output: Args { inner: ["target/debug/rust-cmd", "your", "args", "here"] }
    println!("{:?}", std::env::args());
}
