# Valid Palindrome II

[LeetCode #680](https://leetcode.com/problems/valid-palindrome-ii/description/)

---

### Statement

Given a string `s`, return `true` if the `s` can be palindrome after deleting at most one character from it.

**Constraints:**
- \\( 1 \le \text{s.length} \le 10^5 \\)
- `s` consists of lowercase English letters.

### Examples

**Example 1:**
```text
Input: s = "aba"
Output: true
```

**Example 2:**
```text
Input: s = "abca"
Output: true
Explanation: You could delete the character 'c'.
```

**Example 3:**
```text
Input: s = "abc"
Output: false
```

---

### Solution

Two-pointer approach. Walk inward from both ends. When a mismatch is found, we have exactly one deletion budget left — try skipping either the left character or the right character and check if the remaining substring is a palindrome.

```rust,ignore
impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let mut left = 0;
        let mut right = s.len().saturating_sub(1);

        let s_chars: Vec<char> = s.chars().collect();

        while left <= right {
            if s_chars[left] != s_chars[right] {
                return is_palindrome(&s_chars[left + 1..right + 1])
                    || is_palindrome(&s_chars[left..right]);
            }

            left += 1;
            right = right.saturating_sub(1);
        }

        true
    }
}

fn is_palindrome(s: &[char]) -> bool {
    let mut right = s.len().saturating_sub(1);
    let mut left = 0;

    while left <= right {
        if s[left] != s[right] {
            return false;
        }

        left += 1;
        right = right.saturating_sub(1);
    }

    true
}
```
