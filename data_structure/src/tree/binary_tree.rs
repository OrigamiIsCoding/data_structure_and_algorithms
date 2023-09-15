use std::cell::{RefCell};
use std::rc::Rc;

pub type Node<T> = Rc<RefCell<BinaryTreeNode<T>>>;

pub struct BinaryTreeNode<T> {
    pub value: T,
    pub left: Option<Node<T>>,
    pub right: Option<Node<T>>,
}

impl<T> BinaryTreeNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

pub struct BinaryTree<T> {
    root: Option<Node<T>>,
}

impl<T> BinaryTree<T> {
    pub fn new(root: Option<Node<T>>) -> Self {
        Self { root }
    }
    pub fn root(&self) -> &Option<Node<T>> {
        &self.root
    }
}
