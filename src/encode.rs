use crate::tree_util::HufTreeNode;
use ahash::AHashMap;
use bitvec::prelude::*;
use bitvec::slice as bv_slice;
use std::collections::VecDeque;

pub fn encode(input_data: &[u8]) -> (Vec<HufTreeNode>, Vec<u8>) {
    // frequency analysis
    let mut frequency_map: AHashMap<u8, usize> = AHashMap::new();

    for &c in input_data {
        let count = frequency_map.entry(c).or_insert(0);
        *count += 1;
    }

    // creating huffman tree
    let tree_size = (frequency_map.len() * 2) - 1;
    let mut huffman_tree: Vec<HufTreeNode> = Vec::with_capacity(tree_size);

    for (val, freq) in frequency_map {
        huffman_tree.push(HufTreeNode::new(val, freq, -1, -1));
    }

    huffman_tree.sort_unstable_by_key(|k| k.freq);

    let mut queue1: VecDeque<i16> = (0..(tree_size as i16)).collect();
    let mut queue2: VecDeque<i16> = VecDeque::with_capacity(tree_size);

    let mut j = 0;
    while (queue1.len() + queue2.len()) > 1 {
        let (elem1, freq1) = take_less(&mut queue1, &mut queue2, &huffman_tree);
        let (elem2, freq2) = take_less(&mut queue1, &mut queue2, &huffman_tree);

        huffman_tree.push(HufTreeNode::new(0, freq1 + freq2, elem1, elem2));

        queue2.push_back(j);
        j += 1;
    }

    // create lookup table for codewords
    let mut codewords: BitVec = BitVec::new();
    let mut lookup_table: AHashMap<u8, &BitSlice> = AHashMap::new();

    traverse_tree(
        huffman_tree.len() - 1,
        0,
        &mut codewords,
        &mut lookup_table,
        &huffman_tree,
    );

    // encoding original data

    (Vec::new(), Vec::new())
}

// currently not working
// see https://stackoverflow.com/questions/66289524/rust-collecting-slices-of-a-vec-in-a-recursive-function
fn traverse_tree<'a>(
    cur_index: usize,
    height: i16,
    codewords: &'a mut bitvec::prelude::BitVec,
    lookup_table: &mut AHashMap<u8, &'a BitSlice>,
    huffman_tree: &[HufTreeNode],
) {
    let cur_node = &huffman_tree[cur_index];

    // if the left child is -1, we reached a leaf
    if cur_node.left == -1 {
        let cur_sequence = &codewords[(codewords.len() - 1 - height as usize)..];
        lookup_table.insert(cur_node.val, cur_sequence);
        return;
    }

    // save the current sequence so we can traverse to the right afterwards
    let mut cur_sequence = codewords[(codewords.len() - 1 - height as usize)..].to_bitvec();
    codewords.push(false);
    traverse_tree(
        cur_node.left as usize,
        height + 1,
        codewords, // mutable borrow - argument requires that `*codewords` is borrowed for `'a`
        lookup_table,
        huffman_tree,
    );

    // append the previously saved current sequence
    codewords.extend(&mut cur_sequence); // second mutable borrow occurs here
    codewords.push(true); // third mutable borrow occurs here
    traverse_tree(
        cur_node.right as usize,
        height + 1,
        codewords, // fourth mutable borrow occurs here
        lookup_table,
        huffman_tree,
    );
}

#[inline]
fn take_less(
    queue1: &mut VecDeque<i16>,
    queue2: &mut VecDeque<i16>,
    huffman_tree: &[HufTreeNode],
) -> (i16, usize) {
    match queue1.front() {
        Some(&i1) => {
            let freq1 = huffman_tree[i1 as usize].freq;

            match queue2.front() {
                Some(&i2) => {
                    let freq2 = huffman_tree[i2 as usize].freq;

                    if freq1 <= freq2 {
                        queue1.pop_front();
                        (i1, freq1)
                    } else {
                        queue2.pop_front();
                        (i2, freq2)
                    }
                }
                None => {
                    queue1.pop_front();
                    (i1, freq1)
                }
            }
        }
        None => {
            let i2 = queue2.pop_front().unwrap();
            let freq2 = huffman_tree[i2 as usize].freq;
            (i2, freq2)
        }
    }
}
