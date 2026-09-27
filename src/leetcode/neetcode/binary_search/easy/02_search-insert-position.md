# Search Insert Position

[LeetCode #35](https://leetcode.com/problems/search-insert-position/description/)

---

### Statement

Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be inserted in order.

You must write an algorithm with \\(\mathcal{O}(\log n)\\) runtime complexity.

**Constraints:**
- \\( 1 \le \text{nums.length} \le 10^4 \\)
- \\( -10^4 \le \text{nums}[i] \le 10^4 \\)
- `nums` contains distinct values sorted in ascending order.
- \\( -10^4 \le \text{target} \le 10^4 \\)

### Examples

**Example 1:**
```text
Input: nums = [1,3,5,6], target = 5
Output: 2
```

**Example 2:**
```text
Input: nums = [1,3,5,6], target = 2
Output: 1
```

**Example 3:**
```text
Input: nums = [1,3,5,6], target = 7
Output: 4
```

---

### Solution

#### Initial approach (with early break):

My first version had an early `break` to handle the case where `mid == 0`, since subtracting from a `usize` of `0` panics. Then return based on whether `target < nums[left]`.

```rust,ignore
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid: usize = (left + right) / 2;

            if nums[mid] == target {
                return mid as i32;
            } else if (left == right) {
                break;
            } else if nums[mid] < target {
                left = mid + 1;
            } else {
                if(mid == 0) {
                    break
                }
                right = mid - 1;
            }
        }

        if target < nums[left] {
            left as i32
        } else {
            (left + 1) as i32
        }
    }
}
```

#### Approach 2 (let the loop run its course):

Let the loop run naturally — after it exits, `left` is always the correct insertion position. The `mid == 0` guard is still needed to prevent usize underflow.

```rust,ignore
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid: usize = (left + right) / 2;

            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                left = mid + 1;
            } else {
                if mid == 0 {
                    break
                }
                right = mid - 1;
            }
        }

        left as i32
    }
}
```
