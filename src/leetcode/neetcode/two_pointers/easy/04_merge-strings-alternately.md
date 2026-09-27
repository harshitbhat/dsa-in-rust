# Merge Strings Alternately

[LeetCode #1768](https://leetcode.com/problems/merge-strings-alternately/description/)

---

### Statement

You are given two strings `word1` and `word2`. Merge the strings by adding letters in alternating order, starting with `word1`. If a string is longer than the other, append the additional letters onto the end of the merged string.

Return the merged string.

**Constraints:**
- \\( 1 \le \text{word1.length}, \text{word2.length} \le 100 \\)
- `word1` and `word2` consist of lowercase English letters.

### Examples

**Example 1:**
```text
Input: word1 = "abc", word2 = "pqr"
Output: "apbqcr"
Explanation: Merge alternating: a-p, b-q, c-r
```

**Example 2:**
```text
Input: word1 = "ab", word2 = "pqrs"
Output: "apbqrs"
Explanation: word2 is longer, so "rs" is appended at the end.
```

**Example 3:**
```text
Input: word1 = "abcd", word2 = "pq"
Output: "apbqcd"
Explanation: word1 is longer, so "cd" is appended at the end.
```

---

### Solution

#### Initial approach (two-phase loop):

Pre-allocate a `Vec<char>` of the combined length. Interleave characters up to `min(n1, n2)`, then append the remainder of the longer string in a second pass.

```rust,ignore
use std::cmp;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let n1 = word1.len();
        let n2 = word2.len();

        let mut merged = vec![' '; n1 + n2];

        let word1_ch: Vec<char> = word1.chars().collect();
        let word2_ch: Vec<char> = word2.chars().collect();

        let mut i = 0;
        let mut k = 0;

        while k < cmp::min(n1, n2) {
            merged[i] = word1_ch[k];
            merged[i + 1] = word2_ch[k];

            i += 2;
            k += 1;
        }

        if k == n1 {
            for j in n1..n2 {
                merged[i] = word2_ch[j];
                i += 1;
            }
        } else {
            for j in n2..n1 {
                merged[i] = word1_ch[j];
                i += 1;
            }
        }

        merged.into_iter().collect()
    }
}
```

#### Approach 2 (single loop over bytes):

Operate on raw bytes instead of chars (safe here since the problem guarantees ASCII-only input). A single loop with two guarded `push`es handles both the interleave and the unequal-length remainder naturally.

```rust,ignore
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let b1 = word1.as_bytes();
        let b2 = word2.as_bytes();

        let mut result = Vec::with_capacity(b1.len() + b2.len());

        let mut i = 0;
        while i < b1.len() || i < b2.len() {
            if i < b1.len() { result.push(b1[i]); }
            if i < b2.len() { result.push(b2[i]); }
            i += 1;
        }

        // Safe: inputs are guaranteed ASCII, so the byte vec is valid UTF-8
        unsafe { String::from_utf8_unchecked(result) }
    }
}
```
