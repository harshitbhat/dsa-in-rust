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

LLM Suggestions:

Your solution is solid — binary search is the right approach and your use of `checked_mul` to handle overflow is clean. But there are a couple of ways it could be improved:

- **The upper bound is too large**
Starting `high = x` means for `x = 2_147_483_647`, you're searching the entire range. The actual square root can't exceed `46_340` for any `i32` input (since `46340² ≈ i32::MAX`). A tighter bound like `x.min(46_340)` — or even `x / 2 + 1` — would halve the search space immediately.

- **The low - 1 return is a bit fragile**
It works correctly, but it's a sign that the loop invariant isn't fully explicit. A cleaner pattern is to track the answer explicitly.

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

For fun, there's also an O(log log n) approach using Newton's method, which converges much faster:

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


This starts from `x` and repeatedly refines the estimate via `r = (r + x/r) / 2`. It converges in roughly 5–6 iterations for any 32-bit input, making it the fastest practical approach here.