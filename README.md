# minbpe.rs
minbpe.rs is a lightweight, minimal implementation of Byte Pair Encoding (BPE) in Rust. Inspired by [Andrej Karpathy's minbpe](https://github.com/karpathy/minbpe), this project aims to provide an efficient and easy-to-use BPE implementation for tokenization tasks in natural language processing.

## Installation
To use minbpe.rs, you need to have Rust installed on your system. You can install Rust from the official website.

## Clone the Repository

`git clone https://github.com/yourusername/minbpe.rs.git`
`cd minbpe.rs`
 Build the Project
`cargo build --release`

## Library Integration
Add minbpe to your Cargo.toml:

```
[dependencies]
minbpe = { git = "https://github.com/narensen/minbpe.rs.git" }
```

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
## Results
![Screenshot 2024-08-04 152947](https://github.com/user-attachments/assets/491239a4-3762-4074-972d-54de6b8bfffc)



