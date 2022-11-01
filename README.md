# RUST study project
![image](https://user-images.githubusercontent.com/2369982/198877652-4ffa1586-8c7b-43d4-abed-1f26c271cc26.png)


### Installation

[Install Rust](https://www.rust-lang.org/tools/install)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Using Cargo

- Create project:  
  `cargo new APP_NAME`
- build:  
  `cargo build`
- run:  
  `cargo run`  
  `cargo run -- ARGS HERE`
- test:  
  `cargo test`

## Projects

### zecho

```bash
$ zecho -h

Display a line of text

Usage: zecho [OPTIONS] [TEXT]...

Arguments:
  [TEXT]...  

Options:
  -s             
  -h, --help     Print help information
  -V, --version  Print version information
```

### zcat

```bash
$ zcat -h
Display a line of text

Usage: zcat [OPTIONS] [FILES]...

Arguments:
  [FILES]...  

Options:
  -n, --number                  number all output lines
  -b, --number-non-blank-lines  number nonempty output lines, overrides -n
  -h, --help                    Print help information
  -V, --version                 Print version information
```

