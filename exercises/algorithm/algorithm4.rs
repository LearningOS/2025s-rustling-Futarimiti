/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::{Ord, Ordering};
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // Insert a node into the tree
    fn insert(&mut self, value: T)
    where
        T: Ord,
    {
        use Ordering::*;
        match value.cmp(&self.value) {
            Less => {
                let Some(ref mut left) = self.left else {
                    self.left = Some(Box::new(TreeNode::new(value)));
                    return;
                };
                left.insert(value);
            }
            Greater => {
                let Some(ref mut right) = self.right else {
                    self.right = Some(Box::new(TreeNode::new(value)));
                    return;
                };
                right.insert(value);
            }
            Equal => (), // no dup
        }
    }

    fn search(&self, value: T) -> bool
    where
        T: Ord,
    {
        use Ordering::*;
        let node = match value.cmp(&self.value) {
            Less => &self.left,
            Greater => &self.right,
            Equal => return true,
        };
        node.as_ref().map_or(false, |node| node.search(value))
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        let Some(ref mut node) = self.root else {
            self.root = Some(Box::new(TreeNode::new(value)));
            return;
        };
        node.insert(value);
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool
    where
        T: Ord,
    {
        let Some(ref node) = self.root else {
            return false;
        };
        node.search(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
