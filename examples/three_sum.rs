// 双指针法
use std::cmp::Ordering;

pub struct Solution;
/// The Solution struct represents a solution to the "triplet sum" problem.
/// The Leetcode link is [15.3 3Sum].
///
/// [15.3 3Sum]: <https://leetcode.com/problems/3sum/description/>
///
/// Given an array of integers `nums`, returns a list of all the unique triplets
/// `[nums[i], nums[j], nums[k]]` such that `nums[i] + nums[j] + nums[k] == 0`.
///
/// The returned triplets must be in non-descending order. The solution set
/// must not contain duplicate triplets.
///
/// # Arguments
///
/// * `nums` - A vector of integers.
///
/// # Returns
///
/// A vector of vectors representing the unique triplets whose sum equals zero.
///
/// # Examples
///
/// ```
/// // Example 1:
/// // Input: nums = [-1,0,1,2,-1,-4]
/// // Output: [[-1,-1,2],[-1,0,1]]
/// assert_eq!(three_sum(vec![-1, 0, 1, 2, -1, -4]), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
///
/// // Example 2:
/// // Input: nums = [0,1,1]
/// // Output: []
/// assert_eq!(three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::new());
///
/// // Example 3:
/// // Input: nums = [0,0,0]
/// // Output: [[0,0,0]]
/// assert_eq!(three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
/// ```
///
/// Note: The order of the output and the order of the triplets does not matter.
///
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut nums = nums;
        nums.sort();
        let len = nums.len();
        for i in 0..len {
            if nums[i] > 0 {
                return result;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let (mut left, mut right) = (i + 1, len - 1);
            while left < right {
                match (nums[i] + nums[left] + nums[right]).cmp(&0) {
                    Ordering::Equal => {
                        result.push(vec![nums[i], nums[left], nums[right]]);
                        left += 1;
                        right -= 1;
                        while left < right && nums[left] == nums[left - 1] {
                            left += 1;
                        }
                        while left < right && nums[right] == nums[right + 1] {
                            right -= 1;
                        }
                    }
                    Ordering::Greater => right -= 1,
                    Ordering::Less => left += 1,
                }
            }
        }
        result
    }
}

fn main() {
    // Your main program logic can go here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum() {
        // Example 1
        let nums1 = vec![-1, 0, 1, 2, -1, -4];
        let result1 = Solution::three_sum(nums1);
        assert_eq!(result1, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);

        // Example 2
        let nums2 = vec![0, 1, 1];
        let result2 = Solution::three_sum(nums2);
        assert_eq!(result2, Vec::<Vec<i32>>::new());

        // Example 3
        let nums3 = vec![0, 0, 0];
        let result3 = Solution::three_sum(nums3);
        assert_eq!(result3, vec![vec![0, 0, 0]]);
    }
}
