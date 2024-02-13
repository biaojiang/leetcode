use std::collections::VecDeque;

/// The Solution struct represents a solution to the "Sliding Window Maximum" problem.
/// The Leetcode link is [239. Sliding Window Maximum].
///
/// [239. Sliding Window Maximum]: <https://leetcode.com/problems/sliding-window-maximum/description/>
pub struct Solution;

impl Solution {
    /// Finds the maximum value in each sliding window of size `k` in the given vector `nums`.
    ///
    /// # Arguments
    ///
    /// * `nums` - A vector of integers representing the input array.
    /// * `k` - An integer representing the size of the sliding window.
    ///
    /// # Returns
    ///
    /// A vector containing the maximum value in each sliding window.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::VecDeque;
    /// let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    /// let k = 3;
    /// let result = Solution::max_sliding_window(nums, k);
    /// assert_eq!(result, vec![3, 3, 5, 5, 6, 7]);
    /// ```
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = vec![]; // Initialize the result vector to store maximum values in each window
        let mut queue = VecDeque::with_capacity(k as usize); // Initialize a deque to store indices of elements in the current window

        for (i, &v) in nums.iter().enumerate() {
            // If the queue's length exceeds k, remove the expired element from the front
            // 如果队列长度超过 k，那么需要移除队首过期元素
            if i - queue.front().unwrap_or(&0) == k as usize {
                queue.pop_front();
            }

            // Remove elements from the back of the queue if they are smaller than the current element
            while let Some(&index) = queue.back() {
                if nums[index] >= v {
                    break;
                }
                // 如果队列第一个元素比当前元素小，那么就把队列第一个元素弹出
                queue.pop_back();
            }

            // Push the current index into the deque
            queue.push_back(i);

            // If the current index is greater than or equal to k - 1, add the maximum of the window to the result
            if i >= k as usize - 1 {
                res.push(nums[queue[0]]);
            }
        }

        res // Return the result vector containing maximum values in each window
    }
}

fn main() {
    // Your main program logic can go here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sliding_window() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;
        let expected_output = vec![3, 3, 5, 5, 6, 7];

        let result = Solution::max_sliding_window(nums, k);

        assert_eq!(result, expected_output);
    }
}

