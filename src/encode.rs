use crate::tree_util::{HufTreeNode, ShortHufTreeNode};
use ahash::AHashMap;
use bitvec::prelude::*;
use std::collections::VecDeque;

pub fn encode_data(input_data: &[u8]) -> (Vec<ShortHufTreeNode>, BitVec<Lsb0, u8>, u8) {
    // frequency analysis
    let mut frequency_map: AHashMap<u8, usize> = AHashMap::new();

    for &c in input_data {
        let count = frequency_map.entry(c).or_insert(0);
        *count += 1;
    }

    // creating huffman tree
    let tree_size = (frequency_map.len() * 2) - 1;
    let mut huffman_tree: Vec<HufTreeNode> = Vec::with_capacity(tree_size);

    for (&val, &freq) in frequency_map.iter() {
        huffman_tree.push(HufTreeNode::new(val, freq, -1, -1));
    }

    huffman_tree.sort_unstable_by_key(|k| k.freq);

    let mut queue1: VecDeque<usize> = (0..frequency_map.len()).collect();
    let mut queue2: VecDeque<usize> = VecDeque::with_capacity(frequency_map.len());

    let mut j = huffman_tree.len();
    while (queue1.len() + queue2.len()) > 1 {
        let (elem1, freq1) = take_less(&mut queue1, &mut queue2, &huffman_tree);
        let (elem2, freq2) = take_less(&mut queue1, &mut queue2, &huffman_tree);

        huffman_tree.push(HufTreeNode::new(
            0,
            freq1 + freq2,
            elem1 as i16,
            elem2 as i16,
        ));

        queue2.push_back(j);
        j += 1;
    }

    // create lookup table for codewords
    let mut codewords: BitVec = BitVec::new();
    let mut lookup_table: AHashMap<u8, (usize, usize)> = AHashMap::new();

    traverse_tree(
        huffman_tree.len() - 1,
        0,
        &mut codewords,
        &mut lookup_table,
        &huffman_tree,
    );

    // encoding original data
    let mut bitsequence: BitVec<Lsb0, u8> = BitVec::new();

    for c in input_data {
        let indices = lookup_table.get(c).unwrap();
        let slice = &codewords[indices.0..indices.1 + 1];

        bitsequence.extend_from_bitslice(slice);
    }

    let actual_len = bitsequence.len();
    let aligned_len = (actual_len + 7) & (-8isize) as usize;
    let fillup = aligned_len - actual_len;
    bitsequence.resize(aligned_len, false);
    bitsequence.shift_right(fillup);

    let short_tree: Vec<ShortHufTreeNode> =
        huffman_tree.iter().map(|node| node.to_short()).collect();

    (short_tree, bitsequence, fillup as u8)
}

fn traverse_tree(
    cur_index: usize,
    height: usize,
    codewords: &mut BitVec,
    lookup_table: &mut AHashMap<u8, (usize, usize)>,
    huffman_tree: &[HufTreeNode],
) {
    let cur_node = &huffman_tree[cur_index];

    // if the left child is -1, we reached a leaf
    if cur_node.left == -1 {
        let l_ind = codewords.len() - height;
        let r_ind = codewords.len() - 1;
        lookup_table.insert(cur_node.val, (l_ind, r_ind));
        return;
    }

    // save the current sequence so we can traverse to the right afterwards
    let mut cur_sequence = codewords[(codewords.len() - height as usize)..].to_bitvec();
    codewords.push(false);
    traverse_tree(
        cur_node.left as usize,
        height + 1,
        codewords,
        lookup_table,
        huffman_tree,
    );
    // append the previously saved current sequence
    codewords.append(&mut cur_sequence);

    codewords.push(true);
    traverse_tree(
        cur_node.right as usize,
        height + 1,
        codewords,
        lookup_table,
        huffman_tree,
    );
}

#[inline]
fn take_less(
    queue1: &mut VecDeque<usize>,
    queue2: &mut VecDeque<usize>,
    huffman_tree: &[HufTreeNode],
) -> (usize, usize) {
    match queue1.front() {
        Some(&i1) => {
            let freq1 = huffman_tree[i1].freq;

            match queue2.front() {
                Some(&i2) => {
                    let freq2 = huffman_tree[i2].freq;

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
            let freq2 = huffman_tree[i2].freq;
            (i2, freq2)
        }
    }
}
