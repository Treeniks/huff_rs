# huff_rs

[![Crates.io](https://img.shields.io/crates/v/huff_rs)](https://crates.io/crates/huff_rs)
[![GitHub last commit](https://img.shields.io/github/last-commit/Treeniks/huff_rs)](https://github.com/Treeniks/huff_rs)
[![License](https://img.shields.io/github/license/Treeniks/huff_rs)](https://github.com/Treeniks/huff_rs/blob/master/LICENSE)

This program was created as a fun project of mine after having to implement Huffman coding for a university project. It utilizes the [bitvec](https://crates.io/crates/bitvec) crate to create the bitsequences. The performance of this program is honestly pretty bad, I have yet to do further analysis on that. The file format for the compressed files is something I came up with, so afaik it doesn't conform to any standard (if there even is one).

## Help

```zsh
huff_rs 1.0
Thomas Lindae <thomas.lindae@in.tum.de>
Compresses files with huffman encoding

USAGE:
    huff_rs <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    decode    Decodes the specified file
    encode    Encodes the specified file
    help      Prints this message or the help of the given subcommand(s)
```

## Installation

It's best to use cargo to install `huff_rs`:

```zsh
cargo install huff_rs
```

## Building

To build `huff_rs`:

```zsh
git clone https://github.com/Treeniks/huff_rs
cd huff_rs
cargo build --release
./target/release/huff_rs --version
```

## Usage

### Encode

To encode a file:
```zsh
huff_rs encode file.txt
```
The default output filename will be the same as the input with the extension replaced by `.huff`.\
You can also specify the output filename with `-o`:
```zsh
huff_rs encode file.txt -o compressed.huff
```

### Decode

To decode a file:
```zsh
huff_rs decode file.huff
```
The default output filename will be the same as the input with the extension replaced by `.txt`.\
You can also specify the output filename with `-o`:
```zsh
huff_rs decode file.huff -o original.txt
```

## TODO

There is 2 main things that this project still needs:
1. adding checks for correct file format in decode\
Currently the program doesn't check for the correct file format. As such, if you give it the wrong file format, it will either give a random error or panic.
2. better testing\
I have not implemented proper testing yet, only one small test for a single string.
