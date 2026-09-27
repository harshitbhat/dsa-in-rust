# Valid Palindrome

[LeetCode #125](https://leetcode.com/problems/valid-palindrome/description/)

---

### Statement

A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward.

Given a string `s`, return `true` if it is a palindrome, or `false` otherwise.

**Constraints:**
- \\( 1 \le \text{s.length} \le 2 \times 10^5 \\)
- `s` consists only of printable ASCII characters.

### Examples

**Example 1:**
```text
Input: s = "A man, a plan, a canal: Panama"
Output: true
Explanation: "amanaplanacanalpanama" is a palindrome.
```

**Example 2:**
```text
Input: s = "race a car"
Output: false
Explanation: "raceacar" is not a palindrome.
```

**Example 3:**
```text
Input: s = " "
Output: true
Explanation: s is an empty string after removing non-alphanumeric characters. An empty string reads the same forward and backward.
```

---

### Solution

Filter and normalize the string into a `Vec<char>`, then apply the two-pointer palindrome check. `.saturating_sub(1)` on the `usize` right pointer prevents underflow on empty input.

```rust,ignore
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s
            .trim()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        if chars.is_empty() {
            return true;
        }

        let mut left = 0;
        let mut right = chars.len().saturating_sub(1);

        while left <= right {
            if chars[left] != chars[right] {
                return false;
            }

            left += 1;
            right = right.saturating_sub(1);
        }

        true
    }
}
```
