mod decode;
mod encode;
mod tree_util;

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use decode::decode_data;
use encode::encode_data;
use tree_util::ShortHufTreeNode;

use bitvec::prelude::*;
use byteorder::{LE, ReadBytesExt, WriteBytesExt};
use clap::{Args, Parser, Subcommand};
use throbber::Throbber;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encode(CommandArgs),
    Decode(CommandArgs),
}

#[derive(Args)]
struct CommandArgs {
    input_file: PathBuf,
    #[arg(short)]
    output_file: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let mut throbber = Throbber::new();

    match cli.command {
        Commands::Encode(CommandArgs {
            input_file,
            output_file,
        }) => {
            let output_file =
                output_file.unwrap_or_else(|| input_file.with_added_extension("huff"));

            if let Err(e) = encode_file(&input_file, &output_file, &mut throbber) {
                throbber.fail(e.to_string());
            }
        }
        Commands::Decode(CommandArgs {
            input_file,
            output_file,
        }) => {
            let output_file = output_file.unwrap_or_else(|| {
                if let Some(extension) = input_file.extension()
                    && extension == "huff"
                {
                    input_file.with_extension("")
                } else {
                    input_file.with_added_extension("decoded")
                }
            });

            if let Err(e) = decode_file(&input_file, &output_file, &mut throbber) {
                throbber.fail(e.to_string());
            }
        }
    }

    throbber.end();
}

fn encode_file(
    input_filename: &Path,
    output_filename: &Path,
    throbber: &mut Throbber,
) -> Result<(), std::io::Error> {
    throbber.start_with_msg(format!("Encoding {}", input_filename.display()));

    let input_data = fs::read(input_filename)?;
    let result = encode_data(&input_data);

    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&output_filename)?;

    throbber.success(format!("Encoded {}", input_filename.display()));

    throbber.start_with_msg(format!("Output {}", output_filename.display()));

    output_file.write_u16::<LE>(result.0.len() as u16)?; // huffman_tree size - cannot be larger than a u16
    write_tree(&mut output_file, &result.0)?; // huffman_tree
    output_file.write_u8(result.2)?; // fillup
    output_file.write_all(result.1.as_raw_slice())?; // bitsequence

    throbber.success(format!("Output: {}", output_filename.display()));

    Ok(())
}

fn decode_file(
    input_filename: &Path,
    output_filename: &Path,
    throbber: &mut Throbber,
) -> Result<(), std::io::Error> {
    throbber.start_with_msg(format!("Decoding {}", input_filename.display()));

    let mut input_file = OpenOptions::new().read(true).open(input_filename)?;

    let huffman_tree_size = input_file.read_u16::<LE>()?;
    let huffman_tree = read_tree(&mut input_file, huffman_tree_size)?;
    let fillup = input_file.read_u8()?;

    let mut bitsequence = Vec::new();
    input_file.read_to_end(&mut bitsequence)?;
    let bitsequence = BitVec::from_vec(bitsequence);

    let result = decode_data(&huffman_tree, &bitsequence[fillup as usize..]);

    throbber.success(format!("Encoded {}", input_filename.display()));

    throbber.start_with_msg(format!("Output {}", output_filename.display()));

    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&output_filename)?;

    output_file.write_all(&result)?;

    throbber.success(format!("Output: {}", output_filename.display()));

    Ok(())
}

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

#[cfg(test)]
mod tests {
    use crate::decode::decode_data;
    use crate::encode::encode_data;

    #[test]
    fn encode_decode_same_result() {
        let data = b"sesamstrasse";

        let encode_result = encode_data(data);
        let tree = encode_result.0;
        let bitsequence = encode_result.1;
        let fillup = encode_result.2;

        let decode_result = decode_data(&tree, &bitsequence[fillup as usize..]);

        assert_eq!(&data[..], &decode_result[..]);
    }
}
