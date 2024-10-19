/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,   //Ord 是一个更严格的 trait，它要求所有值之间可以比较大小，必须满足全序关系（即满足自反性、反对称性和传递性
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
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
        //TODO
        if self.root.is_none() {
            self.root = Some(Box::new(TreeNode::new(value)));
            return;
        }
        let root = self.root.as_mut().unwrap();
        if value < root.value {
            if let Some(node) = &mut root.left {
                node.insert(value);
            } else {
                root.left = Some(Box::new(TreeNode::new(value)));
            }
        } else if value > root.value {
            if let Some(node) = &mut root.right {
                node.insert(value);
            } else {
                root.right = Some(Box::new(TreeNode::new(value)));
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        if self.root.is_none() {
            return false;
        }

        if let Some(root) = &self.root {
            if value < root.value {
                if let Some(left_node) = &root.left {
                    left_node.search(value)
                } else {
                    false
                }
            } else if value > root.value {
                if let Some(right_node) = &root.right {
                    right_node.search(value)
                } else {
                    false
                }
            } else {
                true
            }
        } else {
            false
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        if value < self.value {
            if let Some(left_node) = &mut self.left {
                left_node.insert(value);
            } else {
                self.left = Some(Box::new(TreeNode::new(value)));
            }
        } else {
            if let Some(right_node) = &mut self.right {
                right_node.insert(value);
            } else {
                self.right = Some(Box::new(TreeNode::new(value)));
            }
    }
}

    fn search(&self, value: T) -> bool {
        if value < self.value {
            if let Some(left_node) = &self.left {
                left_node.search(value)
            } else {
                false
            }
        } else if value > self.value {
            if let Some(right_node) = &self.right {
                right_node.search(value)
            } else {
                false
            }
        } else {
            true
        }
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
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


