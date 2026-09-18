# Longest Common Prefix

[LeetCode #14](https://leetcode.com/problems/longest-common-prefix/description/)

---

### Statement

Write a function to find the longest common prefix string amongst an array of strings.

If there is no common prefix, return an empty string `""`.

**Constraints:**
- \\( 1 \le \text{strs.length} \le 200 \\)
- \\( 0 \le \text{strs}[i]\text{.length} \le 200 \\)
- `strs[i]` consists of only lowercase English letters if it is non-empty.

### Examples

**Example 1:**
```text
Input: strs = ["flower","flow","flight"]
Output: "fl"
```

**Example 2:**
```text
Input: strs = ["dog","racecar","car"]
Output: ""
Explanation: There is no common prefix among the input strings.
```

---

### Solution

Horizontal scanning approach. Start by assuming the first string is the entire common prefix, then compare it sequentially against every other string in the list:
- Use `.zip()` and `.take_while()` to find the count of matching characters from the start.
- Shrink the `prefix` in place using `.truncate()` to keep only the shared length.
- If at any point the prefix becomes empty, do an early return since no common prefix can exist.

```rust,no_run
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        let mut prefix = strs[0].clone();

        for s in &strs[1..] {
            let common_len = prefix
                .chars()
                .zip(s.chars())
                .take_while(|(a, b)| a == b)
                .count();

            prefix.truncate(common_len);

            if prefix.is_empty() {
                return String::new();
            }
        }

        prefix
    }
}
```