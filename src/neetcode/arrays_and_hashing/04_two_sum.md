# Two Sum

[LeetCode #1](https://leetcode.com/problems/two-sum/description/)

---

### Statement

Given an array of integers `nums` and an integer `target`, return indices of the two numbers such that they add up to `target`.

You may assume that each input would have **exactly one solution**, and you may not use the same element twice.

You can return the answer in any order.

**Constraints:**
- \\( 2 \le \text{nums.length} \le 10^4 \\)
- \\( -10^9 \le \text{nums}[i] \le 10^9 \\)
- \\( -10^9 \le \text{target} \le 10^9 \\)
- **Only one valid answer exists.**

### Examples

**Example 1:**
```text
Input: nums = [2, 7, 11, 15], target = 9
Output: [0, 1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1]
```

**Example 2:**
```text
Input: nums = [3, 2, 4], target = 6
Output: [1, 2]
Explanation: Because nums[1] + nums[2] == 6, we return [1, 2]
```

**Example 3:**
```text
Input: nums = [3, 3], target = 6
Output: [0, 1]
Explanation: Both elements add up to target
```

---

### Solution

- A single-pass `HashMap` approach. For each element, calculate its complement (`target - num`). 
- If the complement exists in the map, return the current index along with the stored index.
- Otherwise, insert the current number and its index into the map.

```rust,ignore
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for i in 0..nums.len() {
            match map.get(&(target - nums[i])) {
                Some(x) => {
                    return vec![i as i32, *x]
                },
                None => {
                    map.insert(nums[i], i as i32);
                }
            }
        }

        vec![]
    }
}
```