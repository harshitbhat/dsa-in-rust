# Contains Duplicate

[LeetCode #217](https://leetcode.com/problems/contains-duplicate/description/)

---

### Statement

Given an integer array `nums`, return `true` if any value appears at least twice in the array, and return `false` if every element is distinct.

**Constraints:**
- \\( 1 \le \text{nums.length} \le 10^5 \\)
- \\( -10^9 \le \text{nums}[i] \le 10^9 \\)

### Examples

**Example 1:**
```text
Input: nums = [1, 2, 3, 1]
Output: true
Explanation: The element 1 occurs at index 0 and index 3
```

**Example 2:**
```text
Input: nums = [1, 2, 3, 4]
Output: false
Explanation: All elements are distinct
```

**Example 3:**
```text
Input: nums = [1, 1, 1, 3, 3, 4, 3, 2, 4, 2]
Output: true
Explanation: Multiple values appear more than once
```

---

### Solution

- My initial thought was to use a `HashMap`, check if the key exists during iteration, and if it does, return `true` immediately.

```rust,ignore
use std::collections::HashMap;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for num in nums {
            match map.get(&num) {
                Some(x) => {
                    return true;
                }
                None => {
                    map.insert(num, 1);
                }
            }
        }
        return false;
    }
}
```

This solution worked but was a bit slow (beat only ~9.8% on LeetCode).

- The second approach was to use a HashSet with pre-allocated capacity. Since `.insert()` returns false if the element is already present, we can do an early return directly inside the loop:

```rust,ignore
use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::with_capacity(nums.len());

        for num in nums {
            if !seen.insert(num) {
                return true;
            }
        }
        return false;
    }
}
```