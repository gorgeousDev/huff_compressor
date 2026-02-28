use std::fmt::{Debug, Formatter};
use super::huff_base_node::HuffBaseNode;

pub struct HuffLeafNode {
    ch: char,
    wt: usize,
}



impl HuffBaseNode for HuffLeafNode {
    fn is_leaf(&self) -> bool { true }
    fn weight(&self) -> usize { self.wt }

    fn character(&self) -> Option<char> {
        Some(self.ch)
    }

}


impl HuffLeafNode {
   pub fn new(node:(char,  usize)) -> HuffLeafNode {
        HuffLeafNode{
            ch: node.0,
            wt: node.1,
        }
    }
}
