# huff_rs

[![Crates.io](https://img.shields.io/crates/v/huff_rs)](https://crates.io/crates/huff_rs)
[![GitHub last commit](https://img.shields.io/github/last-commit/Treeniks/huff_rs)](https://github.com/Treeniks/huff_rs)
[![License](https://img.shields.io/github/license/Treeniks/huff_rs)](https://github.com/Treeniks/huff_rs/blob/master/LICENSE)

I created this for fun after having to implement Huffman coding for a university project. It utilizes the [bitvec](https://crates.io/crates/bitvec) crate to create the bitsequences. The performance of this program is honestly pretty bad. The file format for the compressed files is something arbitrary I came up with.

## Help

```
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

## Building

```sh
git clone https://github.com/Treeniks/huff_rs
cd huff_rs
cargo build --release
./target/release/huff_rs --version
```

## Installation

```sh
cargo install huff_rs
```

## Usage

### Encode

To encode a file:
```sh
huff_rs encode file.txt
```

The default output filename will be the same as the input with the extension replaced by `.huff`. You can also specify the output filename with `-o`:
```sh
huff_rs encode file.txt -o compressed.huff
```

### Decode

To decode a file:
```sh
huff_rs decode file.huff
```
The default output filename will be the same as the input with the extension replaced by `.txt`. You can also specify the output filename with `-o`:
```sh
huff_rs decode file.huff -o original.txt
```

## TODO

- adding checks for correct file format in decode
- tests
