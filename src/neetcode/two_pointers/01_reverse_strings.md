# Reverse String

[LeetCode #344](https://leetcode.com/problems/reverse-string/description/)

---

### Statement

Write a function that reverses a string. The input string is given as an array of characters `s`.

You must do this by modifying the input array in-place with \\(\mathcal{O}(1) \\) extra memory.

**Constraints:**
- \\( 1 \le \text{s.length} \le 10^5 \\)
- `s[i]` is a printable ascii character.

### Examples

**Example 1:**
```text
Input: s = ["h","e","l","l","o"]
Output: ["o","l","l","e","h"]
```

**Example 2:**
```text
Input: s = ["H","a","n","n","a","h"]
Output: ["h","a","n","n","a","H"]
```

---

### Solution

Classic two-pointer approach swapping characters inward from both ends:
- Initialize `left` at index `0` and `right` at index `s.len() - 1`.
- Swap characters at `left` and `right`, then increment `left` and decrement `right`.
- Terminate once the pointers meet or cross each other.

```rust,no_run
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut left = 0;
        let mut right = s.len() - 1;

        while left < right {
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}
```