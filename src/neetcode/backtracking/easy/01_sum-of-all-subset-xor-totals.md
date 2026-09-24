# Sum of All Subset XOR Totals

[LeetCode #1863](https://leetcode.com/problems/sum-of-all-subset-xor-totals/description/)

---

### Statement

The **XOR total** of an array is defined as the bitwise XOR of all its elements, or `0` if the array is empty.

Given an array `nums`, return the **sum of all XOR totals** for every subset of `nums`.

**Note:** Subsets with the same elements should be counted multiple times.

**Constraints:**
- \\( 1 \le \text{nums.length} \le 12 \\)
- \\( 1 \le \text{nums}[i] \le 20 \\)

### Examples

**Example 1:**
```text
Input: nums = [1,3]
Output: 6
Explanation: The 4 subsets of [1,3] are:
  []    → XOR total = 0
  [1]   → XOR total = 1
  [3]   → XOR total = 3
  [1,3] → XOR total = 1 XOR 3 = 2
  Sum = 0 + 1 + 3 + 2 = 6
```

**Example 2:**
```text
Input: nums = [5,1,6]
Output: 28
```

**Example 3:**
```text
Input: nums = [3,4,5,6,7,8]
Output: 480
```

---

### Solution

Backtracking over every subset: at each index, branch into "include this element" (XOR it into the running total) and "exclude it". When the index reaches the end of the array, the running XOR is one subset's contribution — add it to the sum.

```rust,ignore
impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        fn backtrack(nums: &[i32], idx: usize, current_xor: i32) -> i32 {
            if idx == nums.len() {
                return current_xor;
            }
            let include = backtrack(nums, idx + 1, current_xor ^ nums[idx]);
            let exclude = backtrack(nums, idx + 1, current_xor);
            include + exclude
        }
        backtrack(&nums, 0, 0)
    }
}
```
