# Merge Sorted Array

[LeetCode #88](https://leetcode.com/problems/merge-sorted-array/description/)

---

### Statement

You are given two integer arrays `nums1` and `nums2`, sorted in non-decreasing order, and two integers `m` and `n`, representing the number of elements in `nums1` and `nums2` respectively.

Merge `nums1` and `nums2` into a single array sorted in non-decreasing order. The final sorted array should not be returned by the function, but instead be stored inside the array `nums1`. To accommodate this, `nums1` has a length of `m + n`, where the first `m` elements denote the elements that should be merged, and the last `n` elements are set to `0` and should be ignored. `nums2` has a length of `n`.

**Constraints:**
- `nums1.length == m + n`
- `nums2.length == n`
- \\( 0 \le m, n \le 200 \\)
- \\( 1 \le m + n \le 200 \\)
- \\( -10^9 \le \text{nums1}[i], \text{nums2}[j] \le 10^9 \\)

### Examples

**Example 1:**
```text
Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
Output: [1,2,2,3,5,6]
```

**Example 2:**
```text
Input: nums1 = [1], m = 1, nums2 = [], n = 0
Output: [1]
```

**Example 3:**
```text
Input: nums1 = [0], m = 0, nums2 = [1], n = 1
Output: [1]
```

---

### Solution

#### Initial approach (merge from the back):

Three-pointer merge filling `nums1` from right to left to avoid overwriting unmerged elements. The third `while p1` loop is a no-op — remaining `nums1` elements are already in place.

```rust,ignore
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut p1 = m - 1;
        let mut p2 = n - 1;

        let mut p = nums1.len() - 1;

        while p1 >= 0 && p2 >= 0 {
            let idx1 = p1 as usize;
            let idx2 = p2 as usize;
            let idx = p as usize;

            if nums1[idx1] > nums2[idx2] {
                nums1[idx] = nums1[idx1];
                p1 -= 1;
            } else {
                nums1[idx] = nums2[idx2];
                p2 -= 1;
            }
            p -= 1
        }

        while p2 >= 0 {
            let idx2 = p2 as usize;
            let idx = p as usize;

            nums1[idx] = nums2[idx2];
            p2 -= 1;
            p -= 1;
        }
    }
}
```

#### Cleaner version (all-usize, `copy_from_slice` for remainder):

Use `usize` indices throughout to eliminate casts on every iteration. Replace the remainder loop with `copy_from_slice`, which is a no-op when `p2 == 0`.

```rust,ignore
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut p1, mut p2, mut p) = (m as usize, n as usize, (m + n) as usize);

        while p1 > 0 && p2 > 0 {
            p -= 1;
            if nums1[p1 - 1] > nums2[p2 - 1] {
                nums1[p] = nums1[p1 - 1];
                p1 -= 1;
            } else {
                nums1[p] = nums2[p2 - 1];
                p2 -= 1;
            }
        }

        // Copy any remaining nums2 elements; nums1 leftovers are already in place
        nums1[..p2].copy_from_slice(&nums2[..p2]);
    }
}
```
