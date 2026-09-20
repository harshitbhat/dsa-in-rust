# Guess Number Higher or Lower

[LeetCode #374](https://leetcode.com/problems/guess-number-higher-or-lower/description/)

---

### Statement

We are playing a guessing game. I pick a number from `1` to `n`. You guess which number I picked.

Every time you guess wrong, I will tell you whether the number I picked is higher or lower than your guess.

A pre-defined API `guess(num: i32) -> i32` is provided:
- Returns `-1` if your guess is higher than the picked number.
- Returns `1` if your guess is lower than the picked number.
- Returns `0` if your guess is correct.

**Constraints:**
- \\( 1 \le n \le 2^{31} - 1 \\)
- \\( 1 \le \text{pick} \le n \\)

### Examples

**Example 1:**
```text
Input: n = 10, pick = 6
Output: 6
```

**Example 2:**
```text
Input: n = 1, pick = 1
Output: 1
```

**Example 3:**
```text
Input: n = 2, pick = 1
Output: 1
```

---

### Solution

Binary search over the range `[1, n]`. At each step call `guess(mid)` to narrow the window. Use `i64` for `mid` to avoid overflow when computing `(left + right) / 2` at the upper end of `i32`.

```rust,ignore
impl Solution {
    unsafe fn guessNumber(&self, n: i32) -> i32 {
        let mut left: i64 = 1;
        let mut right: i64 = n as i64;

        loop {
            let mid = left + (right - left) / 2;
            match guess(mid as i32) {
                0 => return mid as i32,
                -1 => right = mid - 1,
                _ => left = mid + 1,
            }
        }
    }
}
```
