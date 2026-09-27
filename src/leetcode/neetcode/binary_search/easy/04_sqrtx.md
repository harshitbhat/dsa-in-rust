# Sqrt(x)

[LeetCode #69](https://leetcode.com/problems/sqrtx/description/)

---

### Statement

Given a non-negative integer `x`, return the square root of `x` rounded down to the nearest integer. The returned integer should be non-negative as well.

You must not use any built-in exponent function or operator.

**Constraints:**
- \\( 0 \le x \le 2^{31} - 1 \\)

### Examples

**Example 1:**
```text
Input: x = 4
Output: 2
Explanation: The square root of 4 is 2, so we return 2.
```

**Example 2:**
```text
Input: x = 8
Output: 2
Explanation: The square root of 8 is 2.82842..., and since we round down, 2 is returned.
```

---

### Solution

#### Initial approach (binary search with `checked_mul`):

Binary search over `[0, x]`. Use `.checked_mul()` to guard against `i32` overflow when squaring `mid` near the top of the range.

```rust,ignore
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut low = 0;
        let mut high = x;

        while low <= high {
            let mid: i32 = low + (high - low) / 2;

            match mid.checked_mul(mid) {
                Some(res) => {
                    if res == x {
                        return mid;
                    } else if res > x {
                        high = mid - 1;
                    } else {
                        low = mid + 1;
                    }
                },
                None => {
                    high = mid - 1;
                }
            }
        }

        high
    }
}
```

#### Approach 2 (tighter upper bound):

The square root of any `i32` value can never exceed `46,340` (since \\(46340^2 \approx 2^{31}\\)). Capping `high` at `x.min(46_340)` eliminates the overflow concern entirely and halves the search space for large inputs.

```rust,ignore
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut low = 0;
        let mut high = x.min(46_340);

        while low <= high {
            let mid = low + (high - low) / 2;
            let sq = mid * mid;

            if sq == x {
                return mid;
            } else if sq > x {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }

        high
    }
}
```

#### Alternative: Newton's method

An \\(\mathcal{O}(\log \log n)\\) approach. Repeatedly refine the estimate via `r = (r + x/r) / 2`. Converges in roughly 5–6 iterations for any 32-bit input, making it the fastest practical approach.

```rust,ignore
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 { return x; }
        let x = x as i64;
        let mut r = x;
        while r * r > x {
            r = (r + x / r) / 2;
        }
        r as i32
    }
}
```
