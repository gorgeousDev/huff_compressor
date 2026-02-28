use std::fmt::{Debug, Formatter, Result as FmtResult};

pub trait  HuffBaseNode {
    fn is_leaf(&self)-> bool;
    // fn is_left_child(&self, flag: bool)-> bool{flag}
    fn weight(&self) -> usize;
    fn left(&self) -> Option<&dyn HuffBaseNode> { None }
    fn right(&self) -> Option<&dyn HuffBaseNode> { None }
    fn character(&self) -> Option<char> { None }
    
    fn is_right_child(&self) -> bool { false }
}

