# Verifying an Alien Dictionary

[LeetCode #953](https://leetcode.com/problems/verifying-an-alien-dictionary/description/)

---

### Statement

In an alien language, they use English lowercase letters but in a different order. The alphabet `order` is some permutation of lowercase letters.

Given a list of `words` written in the alien language, return `true` if and only if the words are sorted lexicographically according to `order`.

**Constraints:**
- \\( 1 \le \text{words.length} \le 100 \\)
- \\( 1 \le \text{words}[i]\text{.length} \le 20 \\)
- `order.length == 26`
- All characters in `words[i]` and `order` are English lowercase letters.

### Examples

**Example 1:**
```text
Input: words = ["hello","leetcode"], order = "hlabcdefgijkmnopqrstuvwxyz"
Output: true
```

**Example 2:**
```text
Input: words = ["word","world","row"], order = "worldabcefghijkmnpqstuvxyz"
Output: false
Explanation: "world" > "row" because 'w' comes after 'r' in the given order.
```

**Example 3:**
```text
Input: words = ["apple","app"], order = "abcdefghijklmnopqrstuvwxyz"
Output: false
Explanation: "apple" > "app" — a prefix is always smaller.
```

---

### Solution

Build a rank table mapping each character to its position in `order`. Then translate every word into its rank representation and check that adjacent pairs are non-decreasing using `.windows(2)`.

```rust,ignore
impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut rank = [0u8; 26];

        for (i, c) in order.bytes().enumerate() {
        	rank[(c - b'a') as usize] = i as u8;
        }

        let translated: Vec<Vec<u8>> = words
        	.iter()
        	.map(|word| word.bytes().map(|ch| rank[(ch - b'a') as usize]).collect())
        	.collect();

        translated.windows(2).all(|pair| pair[0] <= pair[1])
    }
}
```
