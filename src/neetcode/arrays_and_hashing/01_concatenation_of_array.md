# Concatenation of Array

[LeetCode #1929](https://leetcode.com/problems/concatenation-of-array/description/) 

---

### Statement

Given an integer array `nums` of length \\( n \\), create an array `ans` of length \\( 2n \\) where `ans[i] == nums[i]` and `ans[i + n] == nums[i]` for \\( 0 \le i < n \\) (0-indexed).

Specifically, `ans` is the concatenation of two `nums` arrays. Return the array `ans`.

**Constraints:** \\( 1 \le n \le 1000 \\), \\( 1 \le \text{nums}[i] \le 1000 \\)

### Examples

**Example 1:**
```text
Input: nums = [1, 2, 1]
Output: [1, 2, 1, 1, 2, 1]
Explanation: ans = [nums[0], nums[1], nums[2], nums[0], nums[1], nums[2]]
```

**Example 2:**
```text
Input: nums = [1, 3, 2, 1]
Output: [1, 3, 2, 1, 1, 3, 2, 1]
Explanation: Direct concatenation of nums with itself
```

---

### Solution

Just initialise a vector of length twice of the given array and then while iterating over the array set the item for the current index `i` and `i + n`.

```rust,no_run
impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();

        let mut result = vec![0; 2 * n];

        for (i, num) in nums.iter().enumerate() {
            result[i] = *num;
            result[i + n] = *num;
        }

        result
    }
}
```