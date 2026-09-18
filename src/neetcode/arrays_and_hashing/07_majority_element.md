# Majority Element

[LeetCode #169](https://leetcode.com/problems/majority-element/description/)

---

### Statement

Given an array `nums` of size `n`, return the majority element.

The majority element is the element that appears more than `⌊n / 2⌋` times. You may assume that the majority element always exists in the array.

**Constraints:**
- \\(n == \text{nums.length} \\)
- \\( 1 \le n \le 5 \times 10^4 \\)
- \\( -10^9 \le \text{nums}[i] \le 10^9 \\)

### Examples

**Example 1:**
```text
Input: nums = [3,2,3]
Output: 3
```

**Example 2:**
```text
Input: nums = [2,2,1,1,1,2,2]
Output: 2
```

---

### Solution

Sorting-based approach. Since the problem guarantees that the majority element appears strictly more than `⌊n / 2⌋` times, sorting the array will always place the majority element across the middle index (`n / 2`), regardless of whether the element occupies the lower half, upper half, or spans across both.

```rust,no_run
impl Solution {
    pub fn majority_element(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        nums[nums.len() / 2]
    }
}
```