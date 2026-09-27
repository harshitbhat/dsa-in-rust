# Last Stone Weight

[LeetCode #1046](https://leetcode.com/problems/last-stone-weight/description/)

---

### Statement

You are given an array of integers `stones` where `stones[i]` is the weight of the `i`th stone.

On each turn, choose the two heaviest stones and smash them together. Given weights `x ≤ y`:
- If `x == y`, both stones are destroyed.
- If `x != y`, the stone of weight `x` is destroyed and the stone of weight `y` gets new weight `y - x`.

Return the weight of the last remaining stone. If there are no stones left, return `0`.

**Constraints:**
- \\( 1 \le \text{stones.length} \le 30 \\)
- \\( 1 \le \text{stones}[i] \le 1000 \\)

### Examples

**Example 1:**
```text
Input: stones = [2,7,4,1,8,1]
Output: 1
Explanation:
  Smash 7 and 8 → 1:  [2,4,1,1,1]
  Smash 2 and 4 → 2:  [2,1,1,1]
  Smash 2 and 1 → 1:  [1,1,1]
  Smash 1 and 1 → 0:  [1]
  Last stone: 1
```

**Example 2:**
```text
Input: stones = [1]
Output: 1
```

---

### Solution

Use a max-heap. On each iteration, pop the two largest stones, push back the difference if non-zero, and repeat until at most one stone remains.

`BinaryHeap::from(stones)` builds the heap in \\(\mathcal{O}(n)\\).

```rust,ignore
use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(stones);

        while heap.len() > 1 {
            let y = heap.pop();
            let x = heap.pop();

            if let (Some(a), Some(b)) = (y, x) {
                if a != b {
                    heap.push(a - b);
                }
            }
        }

        heap.pop().unwrap_or(0)
    }
}
```
