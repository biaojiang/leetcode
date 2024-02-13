use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

/// The Solution struct represents a solution to the "finding the top k frequent elements in an array".
/// The Leetcode link is [347. Top K Frequent Elements].
///
/// [347. Top K Frequent Elements]: <https://leetcode.com/problems/top-k-frequent-elements/description/>
pub struct Solution;

impl Solution {
    /// Finds the top k frequent elements in an array.
    ///
    /// # Arguments
    ///
    /// * `nums` - A vector of integers representing the input array.
    /// * `k` - The number of top frequent elements to find.
    ///
    /// # Returns
    ///
    /// A vector containing the top k frequent elements.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
    /// assert_eq!(result, vec![1, 2]);
    ///
    /// let result = Solution::top_k_frequent(vec![1], 1);
    /// assert_eq!(result, vec![1]);
    /// ```

    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // Create a HashMap to count the frequency of each element
        let mut hash = HashMap::new();

        // Create a BinaryHeap to store the top k frequent elements, using Reverse for max heap
        let mut heap = BinaryHeap::with_capacity(k as usize);

        // Count the frequency of each element in the input array
        nums.into_iter().for_each(|k| {
            *hash.entry(k).or_insert(0) += 1;
        });

        // Iterate over the HashMap entries
        for (k, v) in hash {
            // Check if the heap is at its capacity
            if heap.len() == heap.capacity() {
                // Compare the top element in the heap with the current entry
                if *heap.peek().unwrap() < (Reverse(v), k) {
                    // If the top element is smaller, skip to the next entry
                    continue;
                } else {
                    // If the top element is larger, pop it from the heap
                    heap.pop();
                }
            }

            // Push the current entry into the heap
            heap.push((Reverse(v), k));
        }

        // Convert the BinaryHeap into a Vec and extract the second element of each tuple
        heap.into_iter().map(|(_, k)| k).rev().collect()
    }
}

fn main() {
    // Your main program logic can go here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent() {
        // Test case 1
        let result = Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3, 4, 4, 4, 4], 2);
        assert_eq!(result, vec![4, 1]);

        // Test case 2
        let result = Solution::top_k_frequent(vec![1], 1);
        assert_eq!(result, vec![1]);
    }
}
