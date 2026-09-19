```rust,ignore
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut p1 = m - 1;
        let mut p2 = n - 1;

        let mut p = nums1.len() - 1;

        while p1 >= 0 && p2 >= 0 {
        	let idx1 = p1 as usize;
        	let idx2 = p2 as usize;
        	let idx = p as usize;

        	if nums1[idx1] > nums2[idx2] {
        		nums1[idx] = nums1[idx1];
        		p1 -= 1;
        	} else {
        		nums1[idx] = nums2[idx2];
        		p2 -= 1;
        	}
        	p -= 1
        }

        while p1 >= 0 {
        	let idx1 = p1 as usize;
        	let idx = p as usize;

        	nums1[idx] = nums1[idx1];
        	p1 -= 1;
        	p -= 1;
        }

        while p2 >= 0 {
        	let idx2 = p2 as usize;
        	let idx = p as usize;

        	nums1[idx] = nums2[idx2];
        	p2 -= 1;
        	p -= 1;
        }
    }
}
```


Asked LLM to review, it gave an elegant solution

Your solution is already solid — correct O(m+n) time, O(1) space, merging from the back to avoid overwrites. A few things worth noting:

**Issues with the current code:**

1. **Underflow on `p`** — `p` is `usize`, so decrementing it when it's `0` will panic in debug mode or wrap in release. You get lucky because `p1`/`p2` loops guard it, but it's fragile.
2. **The third `while p1 >= 0` loop is a no-op** — if elements remain in `nums1`, they're already in place. You can drop it entirely.
3. **Redundant `while p1 >= 0` loop** — same reason above.

**Cleaner version:**

```rust,ignore
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut p1, mut p2, mut p) = (m as usize, n as usize, (m + n) as usize);

        while p1 > 0 && p2 > 0 {
            p -= 1;
            if nums1[p1 - 1] > nums2[p2 - 1] {
                nums1[p] = nums1[p1 - 1];
                p1 -= 1;
            } else {
                nums1[p] = nums2[p2 - 1];
                p2 -= 1;
            }
        }

        // If nums2 has leftovers, copy them in
        nums1[..p2].copy_from_slice(&nums2[..p2]);

        // If nums1 has leftovers, they're already in place — nothing to do
    }
}
```

**What changed and why:**

- **All indices are `usize` from the start** — no casts on every iteration, no signed/unsigned mixing.
- **Loop on `> 0` instead of `>= 0`** — avoids usize underflow entirely; use `p1 - 1` to index.
- **Drop the `while p1` loop** — remaining `nums1` elements are already in their correct positions.
- **Replace the `while p2` loop with `copy_from_slice`** — idiomatic, branchless, and expressive. When `p2 == 0` this is a no-op, so no guard needed.


