# N-th Tribonacci Number

[LeetCode #1137](https://leetcode.com/problems/n-th-tribonacci-number/description/)

---

### Statement

The Tribonacci sequence is defined as:

\\( T_0 = 0,\ T_1 = 1,\ T_2 = 1,\ T_{n+3} = T_n + T_{n+1} + T_{n+2} \\)

Given `n`, return the value of \\( T_n \\).

**Constraints:**
- \\( 0 \le n \le 37 \\)

### Examples

**Example 1:**
```text
Input: n = 4
Output: 4
Explanation: T_3 = 0+1+1 = 2, T_4 = 1+1+2 = 4
```

**Example 2:**
```text
Input: n = 25
Output: 1389537
```

---

### Solution

Same rolling-window approach as Fibonacci, extended to three values. At each step compute the sum of `a`, `b`, `c` and shift the window forward.

```rust,ignore
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n < 2 {
        	return n
        }

        if n == 2 {
        	return 1;
        }

        let mut a = 0;
        let mut b = 1;
        let mut c = 1;
        let mut ans = 0;

        let mut index = 3;

        while index <= n {
        	ans = a + b + c;
        	a = b;
        	b = c;
        	c = ans;

        	index += 1;
        }

        ans
    }
}
```
