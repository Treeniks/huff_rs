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
    let mut lookup_table: AHashMap<u8, &BitPtr> = AHashMap::new();

    // encoding original data

    (Vec::new(), Vec::new())
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
