# Contains Duplicate II

[LeetCode #219](https://leetcode.com/problems/contains-duplicate-ii/description/)

---

### Statement

Given an integer array `nums` and an integer `k`, return `true` if there are two distinct indices `i` and `j` in the array such that `nums[i] == nums[j]` and `abs(i - j) <= k`.

**Constraints:**
- \\( 1 \le \text{nums.length} \le 10^5 \\)
- \\( -10^9 \le \text{nums}[i] \le 10^9 \\)
- \\( 0 \le k \le 10^5 \\)

### Examples

**Example 1:**
```text
Input: nums = [1,2,3,1], k = 3
Output: true
```

**Example 2:**
```text
Input: nums = [1,0,1,1], k = 1
Output: true
```

**Example 3:**
```text
Input: nums = [1,2,3,1,2,3], k = 2
Output: false
```

---

### Solution

Sliding window with a `HashSet`. Maintain a set of at most `k` elements representing the current window. For each new element:
- If inserting returns `false`, the element already exists in the window — return `true`.
- Once the window exceeds size `k`, evict the element at `i - k` to keep the window bounded.

```rust,ignore
use std::collections::HashSet;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;

        let mut window = HashSet::with_capacity(k + 1);

        for (i, &num) in nums.iter().enumerate() {
            if !window.insert(num) {
                return true;
            }

            if window.len() > k {
                window.remove(&nums[i - k]);
            }
        }

        false
    }
}
```
