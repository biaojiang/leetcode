/// Definition of a singly-linked list node.
/// pub next: `Option<Box<ListNode>>`: Represents the next node in the linked list.
/// It's wrapped in Option to handle the case where there might not be a next node,
/// and it's a Box to allow for dynamic allocation on the heap.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    /// Value stored in the node.
    pub val: i32,
    /// Next node in the linked list.
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    /// Creates a new ListNode with the given value.
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

/// Removes elements with the specified value from the linked list.
///
/// # Arguments
///
/// * `head` - The head of the linked list.
/// * `val` - The value to be removed from the linked list.
///
/// # Returns
///
/// The head of the linked list after removal.
impl Solution {
    /// Implementation of removing elements
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        dummy_head.next = head;
        let mut cur = dummy_head.as_mut();
        // 使用take()替换std::men::replace(&mut node.next, None)达到相同的效果，并且更普遍易读
        while let Some(nxt) = cur.next.take() {
            if nxt.val == val {
                cur.next = nxt.next;
            } else {
                cur.next = Some(nxt);
                cur = cur.next.as_mut().unwrap();
            }
        }
        // Return dummy_head.next as the result
        dummy_head.next
    }
}

fn main() {
    // Your main program logic can go here.
    // Create a linked list, call the remove_elements method, etc.
}

/// Helper function to create a linked list from a vector of values.
///
/// # Arguments
///
/// * `values` - A vector of values to be used for creating the linked list.
///
/// # Returns
///
/// An `Option<Box<ListNode>>` representing the head of the linked list.
fn create_linked_list(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut current = &mut head;

    for &val in values.iter() {
        *current = Some(Box::new(ListNode::new(val)));
        current = &mut current.as_mut().unwrap().next;
    }

    // println!("{:#?}", head);
    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_elements() {
        // Example usage of create_linked_list for test case setup
        let input = create_linked_list(vec![1, 2, 6, 3, 4, 5, 6]);
        let output = Solution::remove_elements(input, 6);

        // Assert the output is as expected
        assert_eq!(output, create_linked_list(vec![1, 2, 3, 4, 5]));
    }
}
