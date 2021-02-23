mod decode;
mod encode;
mod tree_util;

use decode::decode;
use encode::encode;

use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read, Write};

use tree_util::ShortHufTreeNode;

use bitvec::prelude::*;

#[macro_use]
extern crate clap;
use clap::App;

use byteorder::{ReadBytesExt, WriteBytesExt, LE};

use loading::Loading;

fn write_tree(
    output_file: &mut File,
    huffman_tree: &[ShortHufTreeNode],
) -> Result<(), std::io::Error> {
    for &node in huffman_tree {
        write_node(output_file, node)?;
    }
    Ok(())
}

fn write_node(output_file: &mut File, node: ShortHufTreeNode) -> Result<(), std::io::Error> {
    output_file.write_u8(node.val)?;
    output_file.write_i16::<LE>(node.left)?;
    output_file.write_i16::<LE>(node.right)?;
    Ok(())
}

fn read_tree(input_file: &mut File, size: u16) -> Result<Vec<ShortHufTreeNode>, std::io::Error> {
    let mut result = Vec::new();
    for _i in 0..size {
        let val = input_file.read_u8()?;
        let left = input_file.read_i16::<LE>()?;
        let right = input_file.read_i16::<LE>()?;
        result.push(ShortHufTreeNode::new(val, left, right));
    }
    Ok(result)
}

fn create_output_filename(input_filename: &str, extension: &str) -> String {
    // TODO replace with `rsplit_once` once it's not nightly-only anymore
    // https://github.com/rust-lang/rust/issues/74773

    let v: Vec<&str> = input_filename.split(".").collect();
    let mut output_filename = v[..v.len() - 1].join(".");
    output_filename.push_str(&format!(".{}", extension));
    return output_filename;
}

fn main() -> Result<(), std::io::Error> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut loading = Loading::new();
    loading.start();
    if let Some(matches) = matches.subcommand_matches("encode") {
        let input_filename = matches.value_of("input").unwrap();
        let output_filename = match matches.value_of("output") {
            Some(output_filename) => output_filename.to_string(),
            None => create_output_filename(input_filename, "huf"),
        };

        loading.text(format!("Encoding {}", input_filename));

        let input_data = fs::read(input_filename)?;
        let result = encode(&input_data);

        loading.success(format!("Encoded {}", input_filename));

        loading.text(format!("Output: {}", output_filename));

        let mut output_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&output_filename)?;

        output_file.write_u16::<LE>(result.0.len() as u16)?; // huffmen_tree size - cannot be larger than a u16
        write_tree(&mut output_file, &result.0)?; // huffman_tree
        output_file.write_u8(result.2)?; // fillup
        output_file.write_all(result.1.as_raw_slice())?; // bitsequence

        loading.success(format!("Output: {}", output_filename));
    } else if let Some(matches) = matches.subcommand_matches("decode") {
        let input_filename = matches.value_of("input").unwrap();
        let output_filename = match matches.value_of("output") {
            Some(output_filename) => output_filename.to_string(),
            None => create_output_filename(input_filename, "txt"),
        };

        loading.text(format!("Decoding {}", input_filename));

        let mut input_file = OpenOptions::new().read(true).open(input_filename).unwrap();

        let huffman_tree_size = input_file.read_u16::<LE>()?;
        let huffman_tree = read_tree(&mut input_file, huffman_tree_size)?;
        let fillup = input_file.read_u8()?;

        let mut bitsequence = Vec::new();
        input_file.read_to_end(&mut bitsequence)?;
        let bitsequence = BitVec::from_vec(bitsequence);

        let result = decode(&huffman_tree, &bitsequence[fillup as usize..]);

        loading.success(format!("Decoded {}", input_filename));

        loading.text(format!("Output: {}", output_filename));

        let mut output_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&output_filename)?;

        output_file.write_all(&result)?;

        loading.success(format!("Output: {}", output_filename));
    }
    loading.end();

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::decode::decode;
    use crate::encode::encode;

    #[test]
    fn encode_decode_same_result() {
        let data = b"sesamstrasse";

        let encode_result = encode(data);
        let tree = encode_result.0;
        let bitsequence = encode_result.1;
        let fillup = encode_result.2;

        let decode_result = decode(&tree, &bitsequence[fillup as usize..]);

        assert_eq!(&data[..], &decode_result);
    }
}
