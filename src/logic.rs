use std::collections::{BinaryHeap, HashMap};

use self::encoding_binary_tree::{
    huff_base_node::HuffBaseNode, huff_internal_node::HuffInternalNode,
    huff_leaf_node::HuffLeafNode, huff_tree::HuffTree,
};
use std::cmp::Reverse;
use std::fs;
use std::path::PathBuf;

mod buff_writer;
pub mod encoding_binary_tree;

use self::encoding_binary_tree::*;

pub fn print_help() {
    let help_message = "\
    Help Message \n\
    todo!";

    println!("{help_message}")
}

pub fn compress(path_buf: PathBuf) {
    let file_text = fs::read_to_string(&path_buf).unwrap();

    let freqs = get_freqs(&file_text);
    let priority_vec = self::encoding_binary_tree::get_priority_vec(&freqs);

    let mut heap: BinaryHeap<Reverse<HuffTree>> = priority_vec
        .iter()
        .map(|f| HuffTree::new_leaf(f.0, f.1))
        .into_iter()
        .map(Reverse)
        .collect();

    let prime_tree = build_tree(&mut heap);
    let mut charcter_codes: HashMap<char, (Vec<bool>, u32)> = HashMap::new(); // Stores (code_bits, code_length)
    let initial_code: Vec<bool> = Vec::new();
    let initial_length: u32 = 0;

    // Call get_characters_codes to populate the charcter_codes map
    get_characters_codes(
        prime_tree.root.as_ref(), // Pass the root of the tree as a trait object reference
        &initial_code,
        initial_length,
        &mut charcter_codes,
    );

    let file_name = "example.huff";
    let mut buff_writer = self::buff_writer::BuffWriter::new(file_name).unwrap();
    let mut bytes = Vec::new();
    buff_writer.serialize(prime_tree.root.as_ref(), &mut bytes);
    buff_writer.write_header(&mut bytes).unwrap();
    
}


fn get_characters_codes(
    node: &dyn HuffBaseNode,
    current_code: &Vec<bool>,
    code_length: u32,
    codes_map: &mut HashMap<char, (Vec<bool>, u32)>, // (code_bits, code_length)
) {
    if node.is_leaf() {
        // Base case: If it's a leaf node, we've found a character.
        // Store its code (current_code and its length) in the map.

        codes_map.insert(
            node.character().unwrap(),
            (current_code.clone(), code_length),
        );
    } else {
        // Recursive step: Traverse left child (convention: '1')
        // Shift current code left by 1 and set the new LSB to 1
        // Increment code length
        if let Some(left_child) = node.left() {
            let mut new_code = current_code.clone();
            new_code.push(false);
            get_characters_codes(left_child, &new_code, code_length + 1, codes_map);
        }

        // Recursive step: Traverse right child (convention: '0')
        // Shift current code left by 1 and set the new LSB to 0 (which is default after shift)
        // Increment code length
        if let Some(right_child) = node.right() {
            let mut new_code = current_code.clone();
            new_code.push(true);
            get_characters_codes(right_child, &new_code, code_length + 1, codes_map);
        }
    }
}

fn build_tree(heap: &mut BinaryHeap<Reverse<HuffTree>>) -> HuffTree {
    // Handle the special case where there's only one unique character
    if heap.len() == 1 {
        let Reverse(single_tree) = heap.pop().unwrap();
        // Create a dummy node with 0 weight
        let dummy_leaf = HuffTree::new_leaf('\0', 0); // Using null character as dummy
        // Create a parent node with the single tree and the dummy node
        let combined_weight = single_tree.root.weight() + dummy_leaf.root.weight();
        let parent =
            HuffTree::new_branch(Box::new(dummy_leaf), Box::new(single_tree), combined_weight);
        return parent;
    }

    // Standard Huffman tree building for normal cases
    while heap.len() > 1 {
        let Reverse(left) = heap.pop().unwrap(); // فك Reverse
        let Reverse(right) = heap.pop().unwrap();

        let combined_weight = left.root.weight() + right.root.weight();
        let parent = HuffTree::new_branch(Box::new(left), Box::new(right), combined_weight);

        heap.push(Reverse(parent));
    }

    let Reverse(final_tree) = heap.pop().unwrap();
    final_tree.print();
    final_tree
}
