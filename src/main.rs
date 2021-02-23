mod decode;
mod encode;
mod tree_util;

use decode::decode;
use encode::encode;

use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

#[macro_use]
extern crate clap;
use clap::{App};

unsafe fn any_as_u8_slice<T: Sized>(x: &T) -> &[u8] {
    std::slice::from_raw_parts((x as *const T) as *const u8, std::mem::size_of::<T>())
}

unsafe fn slice_as_u8_slice<T: Sized>(x: &[T]) -> &[u8] {
    std::slice::from_raw_parts(
        (x as *const [T]) as *const u8,
        x.len() * std::mem::size_of::<T>(),
    )
}

fn main() -> Result<(), std::io::Error> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("encode") {
        let input_filename = matches.value_of("input").unwrap();
        let output_filename = matches.value_of("output").unwrap_or("output.huf");

        let input_data = fs::read(input_filename)?;
        let result = encode(&input_data);
        let mut output_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(output_filename)
            .unwrap();

        output_file.write_all(unsafe { any_as_u8_slice(&input_data.len()) })?; // original file's length
        output_file.write_all(unsafe { any_as_u8_slice(&(result.0.len() as u16)) })?; // size of huffman_tree
        output_file.write_all(unsafe { slice_as_u8_slice(&result.0) })?; // huffman_tree
        output_file.write_all(&[result.2])?; // fillup
        output_file.write_all(result.1.as_raw_slice())?; // bitsequence
    } else if let Some(matches) = matches.subcommand_matches("decode") {
        let input_filename = matches.value_of("input").unwrap();
        let output_filename = matches.value_of("output").unwrap_or("output.huf");

        let input_data = fs::read(input_filename)?;
        // TODO extract tree and bitsequence and pass it to decode
    }

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
