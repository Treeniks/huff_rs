mod decode;
mod encode;
mod tree_util;

use encode::encode;
use decode::decode;

fn main() {
    let result = encode(b"sesamstrasse");
    println!("{:?}", result.1);
}
