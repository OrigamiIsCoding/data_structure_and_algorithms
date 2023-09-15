use data_structure::tree::{BinaryTree, Node};
use crate::tree::tree_traversal::Traversal;

fn inorder_impl<T>(root: &Option<Node<T>>, visited: fn(&T)) {
    if let Some(node) = root {
        inorder_impl(&node.borrow().left, visited);
        visited(&node.borrow().value);
        inorder_impl(&node.borrow().right, visited);
    }
}

fn postorder_impl<T>(root: &Option<Node<T>>, visited: fn(&T)) {
    if let Some(node) = root {
        inorder_impl(&node.borrow().left, visited);
        inorder_impl(&node.borrow().right, visited);
        visited(&node.borrow().value);
    }
}

fn preorder_impl<T>(root: &Option<Node<T>>, visited: fn(&T)) {
    if let Some(node) = root {
        visited(&node.borrow().value);
        inorder_impl(&node.borrow().left, visited);
        inorder_impl(&node.borrow().right, visited);
    }
}

impl<T> Traversal<T> for BinaryTree<T> {
    fn preorder(&self, visited: fn(&T)) {
        preorder_impl(self.root(), visited);
    }
    fn inorder(&self, visited: fn(&T)) {
        inorder_impl(self.root(), visited);
    }

    fn postorder(&self, visited: fn(&T)) {
        inorder_impl(self.root(), visited);
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use data_structure::tree::{BinaryTree, BinaryTreeNode};
    use crate::tree::tree_traversal::Traversal;

    #[test]
    fn test_traversal_inorder() {
        let mut root = BinaryTreeNode::new(1);
        let l = BinaryTreeNode::new(2);
        let r = BinaryTreeNode::new(3);
        root.left = Some(Rc::new(RefCell::new(l)));
        root.right = Some(Rc::new(RefCell::new(r)));

        let tree = BinaryTree::new(Some(Rc::new(RefCell::new(root))));

        tree.inorder(|&i| {
            println!("{i}");
        });
    }
}
