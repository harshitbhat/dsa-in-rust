# Remove Element

[LeetCode #27](https://leetcode.com/problems/remove-element/description/)

---

### Statement

Given an integer array `nums` and an integer `val`, remove all occurrences of `val` in `nums` **in-place**. The order of the elements may be changed. Then return the number of elements in `nums` which are not equal to `val`.

Consider the number of elements in `nums` which are not equal to `val` be `k`, to get accepted, you need to do the following things:
- Change the array `nums` such that the first `k` elements of `nums` contain the elements which````markdown
# Remove Element

[LeetCode #27](https://leetcode.com/problems/remove-element/description/)

---

### Statement

Given an integer array `nums` and an integer `val`, remove all occurrences of `val` in `nums` in-place. The relative order of the remaining elements may be changed.

Return the number of elements in `nums` that are not equal to `val` (let this count be `k`). The first `k` elements of `nums` must hold these remaining values, while whatever remains beyond that index does not matter.

**Constraints:**
- \\( 0 \le \text{nums.length} \le 100 \\)
- \\( 0 \le \text{nums}[i] \le 50 \\)
- \\( 0 \le \text{val} \le 100 \\)

### Examples

**Example 1:**
```text
Input: nums = [3,2,2,3], val = 3
Output: 2, nums = [2,2,_,_]
Explanation: Your function should return k = 2, with the first two elements of nums being 2.
```

**Example 2:**
```text
Input: nums = [0,1,2,2,3,0,4,2], val = 2
Output: 5, nums = [0,1,4,0,3,_,_,_]
Explanation: Your function should return k = 5, with the first five elements of nums containing 0, 0, 1, 3, and 4.
```

---

### Solution

Two-pointer swapping approach from both ends. Since relative order does not need to be preserved, we can minimize operations whenever a target value is spotted:
- If `nums[left]` matches `val`, decrement `right` and swap the matching element with the one at `right`. We do not increment `left` here because the swapped-in value still needs validation.
- If `nums[left]` is different from `val`, advance `left`.
- Once `left` meets `right`, all remaining valid elements reside within `0..left`.

```rust,ignore
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            if nums[left] == val {
                right -= 1;
                nums.swap(left, right);
            } else {
                left += 1;
            }
        }

        left as i32
    }
}
```