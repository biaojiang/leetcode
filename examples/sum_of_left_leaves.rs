use std::cell::RefCell;
use std::rc::Rc;

/// Defines a generic struct named TreeNode with a type parameter `T`.
/// The Leetcode link is [404. Sum of Left Leaves].
///
/// [404. Sum of Left Leaves]: <https://leetcode.com/problems/sum-of-left-leaves/description/>
pub struct TreeNode<T> {
    /// `T` Holds the value of the node
    pub val: T,

    /// Define the left child of the current node
    /// It is an Option because a node may not have a left child.
    /// The left child is wrapped in `Rc<RefCell<TreeNode<T>>>`.
    /// `Rc` stands for "reference counting," and `RefCell` provides interior mutability,
    /// allowing for mutable references even while the struct is immutable.
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

/// Implement TreeNode new.
impl<T> TreeNode<T> {
    #[inline]
    /// Creates a new tree node with the given value.
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/// Solution struct
pub struct Solution;

/// Calculates the sum of values of left leaves in a binary tree.
///
/// Given the root of a binary tree, this function calculates the sum of the values of its left leaves.
///
/// # Arguments
///
/// * `root` - The root of the binary tree represented as an `Option<Rc<RefCell<TreeNode<T>>>>`.
///
/// # Returns
///
/// The sum of the values of left leaves in the binary tree.
///
/// # Example
///
/// ```
/// use std::rc::Rc;
/// use std::cell::RefCell;
///
/// // Construct the binary tree: [3,9,20,null,null,15,7]
/// let root = Some(Rc::new(RefCell::new(TreeNode {
///     val: 3,
///     left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
///     right: Some(Rc::new(RefCell::new(TreeNode {
///         val: 20,
///         left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
///         right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
///     }))),
/// })));
///
/// let result = sum_of_left_leaves(root);
///
/// assert_eq!(result, 24);
/// ```
impl Solution {
    /// Implemention the sum_of_left_leaves method with recursive algorithm.
    pub fn sum_of_left_leaves<T>(root: Option<Rc<RefCell<TreeNode<T>>>>) -> T
    where
        T: std::ops::Add<Output = T> + std::ops::AddAssign + Default + Copy,
    {
        let mut res = T::default();
        if let Some(node) = root {
            if let Some(left) = &node.borrow().left {
                if left.borrow().left.is_none() && left.borrow().right.is_none() {
                    res += left.borrow().val;
                }
            }
            res + Self::sum_of_left_leaves(node.borrow().left.clone())
                + Self::sum_of_left_leaves(node.borrow().right.clone())
        } else {
            T::default()
        }
    }
}

fn main() {
    // Your main program logic can go here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_left_leaves() {
        // Construct the binary tree: [3,9,20,null,null,15,7]
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));

        // Call the function
        let result = Solution::sum_of_left_leaves(root);

        // Assert the result
        assert_eq!(result, 24);
    }

    #[test]
    fn test_sum_of_left_leaves_single_node() {
        // Construct the binary tree with a single node: [1]
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));

        let result = Solution::sum_of_left_leaves(root);

        // Since there is only one node, the sum of left leaves is 0
        assert_eq!(result, 0);
    }
}
