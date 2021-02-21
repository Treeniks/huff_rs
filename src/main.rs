mod decode;
mod encode;
mod tree_util;

use decode::decode;
use encode::encode;

use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

unsafe fn any_as_u8_slice<T: Sized>(x: &T) -> &[u8] {
    std::slice::from_raw_parts((x as *const T) as *const u8, std::mem::size_of::<T>())
}

unsafe fn slice_as_u8_slice<T: Sized>(x: &[T]) -> &[u8] {
    std::slice::from_raw_parts(
        (x as *const [T]) as *const u8,
        x.len() * std::mem::size_of::<T>(),
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // println!("{}", std::mem::size_of::<tree_util::ShortHufTreeNode>());

    let input_filename = &args[1];
    let output_filename = &args[2];

    let input_data = fs::read(input_filename).expect("Something went wrong reading the file");

    let result = encode(&input_data);

    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(output_filename)
        .unwrap();

    output_file.write(unsafe { any_as_u8_slice(&input_data.len()) });
    output_file.write(unsafe { any_as_u8_slice(&(result.0.len() as u16)) });
    output_file.write(unsafe { slice_as_u8_slice(&result.0) });
    // println!("{:?}", result.1);
}
