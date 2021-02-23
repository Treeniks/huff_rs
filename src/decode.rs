use crate::tree_util::ShortHufTreeNode;

use bitvec::prelude::*;

pub fn decode(tree: &[ShortHufTreeNode], input_data: &BitSlice<Lsb0, u8>) -> Vec<u8> {
    let mut result = Vec::new();

    let root_node = tree[tree.len() - 1];
    let mut cur_node = root_node;
    for i in input_data {
        if i == false {
            cur_node = tree[cur_node.left as usize];
        } else {
            cur_node = tree[cur_node.right as usize];
        }

        if cur_node.left == -1 {
            result.push(cur_node.val);
            cur_node = root_node;
        }
    }

    return result;
}
