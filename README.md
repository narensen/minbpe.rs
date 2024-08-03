# bpe.rs
minbpe.rs is a lightweight, minimal implementation of Byte Pair Encoding (BPE) in Rust. Inspired by Andrej Karpathy's minbpe and ash-01xor's bpe.c, this project aims to provide an efficient and easy-to-use BPE implementation for tokenization tasks
in natural language processing.

## Installation
To use minbpe.rs, you need to have Rust installed on your system. You can install Rust from the official website.

## Clone the Repository

`git clone https://github.com/yourusername/minbpe.rs.git`
`cd minbpe.rs`

## Build the Project
`cargo build --release`

## Usage
`cargo run -- encode <input_file> <output_file>`
`cargo run -- decode <input_file> <output_file>`

# Library Integration
Add minbpe to your Cargo.toml:

[dependencies]
minbpe = { git = "https://github.com/yourusername/minbpe.rs.git" }

Use the library in your Rust code:
```
extern crate minbpe;

use minbpe::BPE;

fn main() {
    let mut bpe = BPE::new();
    bpe.train("path/to/textfile.txt");
    let encoded = bpe.encode("text to encode");
    println!("Encoded: {:?}", encoded);
}
```
Acknowledgements
Inspired by minbpe by Andrej Karpathy.
Inspired by bpe.c by ash-01xor.
