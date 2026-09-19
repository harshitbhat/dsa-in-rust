### Solution

```rust,ignore
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s
            .trim()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        let mut left = 0;
        let mut right = chars.len().saturating_sub(1);

        if chars.is_empty() {
            return true;
        }

        while left <= right {
            if chars[left] != chars[right] {
                return false;
            }

            left += 1;
            right = right.saturating_sub(1);;
        }

        true
    }
}
```