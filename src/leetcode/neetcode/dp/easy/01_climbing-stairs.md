# Climbing Stairs

[LeetCode #70](https://leetcode.com/problems/climbing-stairs/description/)

---

### Statement

You are climbing a staircase that takes `n` steps to reach the top. Each time you can climb either `1` or `2` steps. In how many distinct ways can you climb to the top?

**Constraints:**
- \\( 1 \le n \le 45 \\)

### Examples

**Example 1:**
```text
Input: n = 2
Output: 2
Explanation: 1+1, 2
```

**Example 2:**
```text
Input: n = 3
Output: 3
Explanation: 1+1+1, 1+2, 2+1
```

---

### Solution

The number of ways to reach step `n` equals the number of ways to reach step `n-1` plus the number of ways to reach step `n-2` — this is exactly the Fibonacci sequence. Track only the two previous values instead of a full DP table.

```rust,ignore
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }

        let mut a = 1;
        let mut b = 2;
        let mut c = 0;

        let mut i = 3;

        while i <= n {
            c = a + b;
            a = b;
            b = c;
            i += 1;
        }

        c
    }
}
```
