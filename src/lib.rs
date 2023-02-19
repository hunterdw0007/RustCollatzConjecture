use std::{rc::Rc, cell::RefCell};

#[derive(Debug)]
struct BinaryTree {
    pub value: u128,
    pub first_child: Option<BinaryTreeRef>,
    pub second_child: Option<BinaryTreeRef>,
}

type BinaryTreeRef = Rc<RefCell<BinaryTree>>;

impl BinaryTree {
    /// Return an Option reference to the BinaryTree child of this BinaryTree that
    /// contains val or None if is not anywhere in the BinaryTree 
    pub fn find(&self, val:&u128) {
       
    }
}
