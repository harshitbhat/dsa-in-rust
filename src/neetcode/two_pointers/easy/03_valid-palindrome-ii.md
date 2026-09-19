### Solution

```rust,ignore
impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let mut left = 0;
        let mut right = s.len().saturating_sub(1);

        let mut s_chars: Vec<char> = s.chars().collect();

        while(left <= right) {
        	if s_chars[left] != s_chars[right] {
        		return is_palindrome(&s_chars[left+1..right+1]) || is_palindrome(&s_chars[left..right]);
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