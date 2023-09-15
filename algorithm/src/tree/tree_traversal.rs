pub trait Traversal<T> {
    fn preorder(&self, visited: fn(&T));
    fn inorder(&self, visited: fn(&T));
    fn postorder(&self, visited: fn(&T));
}