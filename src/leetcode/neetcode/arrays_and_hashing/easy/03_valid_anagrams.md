# Valid Anagram

[LeetCode #242](https://leetcode.com/problems/valid-anagram/description/)

---

### Statement

Given two strings `s` and `t`, return `true` if `t` is an anagram of `s`, and `false` otherwise.

An **Anagram** is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

**Constraints:**
- \\( 1 \le \text{s.length}, \text{t.length} \le 5 \times 10^4 \\)
- `s` and `t` consist of lowercase English letters.

### Examples

**Example 1:**
```text
Input: s = "anagram", t = "nagaram"
Output: true
Explanation: Both strings contain the exact same characters with identical counts
```

**Example 2:**
```text
Input: s = "rat", t = "car"
Output: false
Explanation: t contains 'c' which is not in s, and lacks 'r'
```

---

### Solution

#### Initial approach (HashMap count & decrement):
  Count frequencies of characters in `s`. Then iterate through `t`:
  - If a character exists in the map, decrement its count (removing the key once it hits `0`).
  - If a character is missing, return `false` early.
  - Finally, check if the map is empty (`map.is_empty()`).

```rust,ignore
use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut map: HashMap<char, i32> = HashMap::new();

        for ch in s.chars() {
            map.entry(ch).and_modify(|val| *val += 1).or_insert(1);
        }
        
        for ch in t.chars() {
            match map.get(&ch) {
                Some(x) => {
                    if *x == 1 {
                        map.remove(&ch);
                    } else {
                        map.insert(ch, x - 1);
                    }
                },
                None => {
                    return false
                }
            }
        }

        if map.len() == 0 { true } else { false }
    }
}
```

This was accepted, beats about 33% os submission, also i miss an case of early return in case the strings are not of equal lengths. 

#### Approach 2 (Fixed-Size Array / Frequency Counter)

Same frequency-counting concept, but significantly faster. Since the problem guarantees lowercase English letters (`a`–`z`), we can use a fixed-size stack array `[0; 26]` instead of a heap-allocated `HashMap`. Iterating with `.bytes()` avoids UTF-8 decoding overhead.

```rust,ignore
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut counts = [0; 26];

        for b in s.bytes() {
            counts[(b - b'a') as usize] += 1;
        }

        for b in t.bytes() {
            let idx = (b - b'a') as usize;
            if counts[idx] == 0 {
                return false;
            }
            counts[idx] -= 1;
        }

        true
    }
}
```

This one was way faster beats 100%