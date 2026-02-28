use std::fmt::{Debug, Formatter};
use super::huff_base_node::HuffBaseNode;

pub struct HuffInternalNode{
    wt: usize,
    left: Box<dyn HuffBaseNode>,
    right: Box<dyn HuffBaseNode>,
    
}



impl HuffBaseNode for HuffInternalNode {
    fn is_leaf(&self)-> bool {false}
    fn weight(&self) -> usize{self.wt}

    fn left(&self) -> Option<&dyn HuffBaseNode> {
        Some(self.left.as_ref())
    }

    fn right(&self) -> Option<&dyn HuffBaseNode> {
        Some(self.right.as_ref())
    }



}

impl HuffInternalNode {
    pub fn new(l_node: Box<dyn HuffBaseNode>,r_node: Box<dyn HuffBaseNode>, wt: usize) -> HuffInternalNode {
        HuffInternalNode{
            left: l_node,
            right: r_node,
            wt
        }
    }
}