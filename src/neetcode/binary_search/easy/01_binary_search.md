# Binary Search

[LeetCode #704](https://leetcode.com/problems/binary-search/description/)

---

### Statement

Given an array of integers `nums` which is sorted in ascending order, and an integer `target`, write a function to search `target` in `nums`. If `target` exists, return its index. Otherwise, return `-1`.

You must write an algorithm with \\(\mathcal{O}(\log n)\\) runtime complexity.

**Constraints:**
- \\( 1 \le \text{nums.length} \le 10^4 \\)
- \\( -10^4 < \text{nums}[i], \text{target} < 10^4 \\)
- All integers in `nums` are unique.
- `nums` is sorted in ascending order.

### Examples

**Example 1:**
```text
Input: nums = [-1,0,3,5,9,12], target = 9
Output: 4
Explanation: 9 exists in nums and its index is 4
```

**Example 2:**
```text
Input: nums = [-1,0,3,5,9,12], target = 2
Output: -1
Explanation: 2 does not exist in nums so return -1
```

---

### Solution

Classic binary search. Maintain two pointers `left` and `right` delimiting the search window. At each step, check the middle element:
- If it equals `target`, return the index.
- If it's less than `target`, move `left` to `mid + 1`.
- If it's greater, move `right` to `mid - 1`.
- If the window empties without a match, return `-1`.

```rust,ignore
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid: usize = (left + right) / 2;

            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        -1
    }
}
```
