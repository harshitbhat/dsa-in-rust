# Remove Duplicates from Sorted Array

[LeetCode #26](https://leetcode.com/problems/remove-duplicates-from-sorted-array/description/)

---

### Statement

Given an integer array `nums` sorted in non-decreasing order, remove the duplicates **in-place** such that each unique element appears only once. The relative order of the elements should be kept the same. Then return the number of unique elements in `nums`.

Consider the number of unique elements of `nums` to be `k`. To get accepted:
- Change the array `nums` such that the first `k` elements of `nums` contain the unique elements in the order they were present originally.
- The remaining elements of `nums` do not matter.
- Return `k`.

**Constraints:**
- \\( 1 \le \text{nums.length} \le 3 \times 10^4 \\)
- \\( -100 \le \text{nums}[i] \le 100 \\)
- `nums` is sorted in non-decreasing order.

### Examples

**Example 1:**
```text
Input: nums = [1,1,2]
Output: 2, nums = [1,2,_]
Explanation: Your function should return k = 2, with the first two elements of nums being 1 and 2.
```

**Example 2:**
```text
Input: nums = [0,0,1,1,1,2,2,3,3,4]
Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
Explanation: Your function should return k = 5, with the first five elements of nums being 0, 1, 2, 3, and 4.
```

---

### Solution

Two-pointer approach. `k` is the write pointer (starts at 1 since the first element is always unique). `i` scans forward. Whenever a new distinct element is encountered, swap it into position `k` and advance both pointers.

```rust,ignore
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut k = 1;
        let mut i = 1;
        let mut current_elem = nums[0];

        while i < nums.len() {
            if nums[i] != current_elem {
                current_elem = nums[i];
                nums.swap(i, k);
                i += 1;
                k += 1;
            } else {
                i += 1;
            }
        }

        k as i32
    }
}
```
