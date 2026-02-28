use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use crate::logic::encoding_binary_tree::huff_internal_node::HuffInternalNode;
use super::huff_base_node::HuffBaseNode;
use super::huff_leaf_node::HuffLeafNode;





pub struct HuffTree{
    pub(crate) root: Box<dyn HuffBaseNode>,
    
}


impl HuffTree {
    pub fn new_leaf(c: char, wt: usize) -> HuffTree {
        let root = HuffLeafNode::new((c, wt));
        HuffTree{root: Box::new(root)}
    }
    pub fn new_branch(l: Box<dyn HuffBaseNode>, r: Box<dyn HuffBaseNode>, wt: usize) -> HuffTree {
        let root = HuffInternalNode::new(l, r, wt);
        HuffTree{root: Box::new(root)}
    }

    pub fn print(&self) {
        Self::print_node(self.root.as_ref(), 0);
    }
    fn print_node(node: &dyn HuffBaseNode, depth: usize) {
        for _ in 0..depth {
            print!("  ");
        }

        if node.is_leaf() {
            println!("Leaf: {:?} ({})", node.character(), node.weight());
        } else {
            println!("Internal ({})", node.weight());
            if let Some(left) = node.left() {
                Self::print_node(left, depth + 1);
            }
            if let Some(right) = node.right() {
                Self::print_node(right, depth + 1);
            }
        }
    }

    pub fn new_from_prepared_root(root: Box<dyn HuffBaseNode>) -> HuffTree {
        HuffTree{root}
    }
}

impl Eq for HuffTree {}

impl PartialEq<Self> for HuffTree {
    fn eq(&self, other: &Self) -> bool {
        self.root.weight() == other.root.weight()
    }
}

impl PartialOrd<Self> for HuffTree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.root.weight().partial_cmp(&other.root.weight())
    }
}

impl Ord for HuffTree{
    fn cmp(&self, other: &Self) -> Ordering {
        other.root.weight().cmp(&self.root.weight())
    }
}


impl HuffBaseNode for HuffTree {
    fn is_leaf(&self) -> bool {
        self.root.is_leaf()
    }

    fn weight(&self) -> usize {
        self.root.weight()
    }

    fn left(&self) -> Option<&dyn HuffBaseNode> {
        self.root.left()
    }

    fn right(&self) -> Option<&dyn HuffBaseNode> {
        self.root.right()
    }

    fn character(&self) -> Option<char> {
        self.root.character()
    }
}

