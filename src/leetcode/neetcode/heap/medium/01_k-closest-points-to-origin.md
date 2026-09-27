# K Closest Points to Origin

[LeetCode #973](https://leetcode.com/problems/k-closest-points-to-origin/description/)

---

### Statement

Given an array of `points` where `points[i] = [xi, yi]` and an integer `k`, return the `k` closest points to the origin `(0, 0)`. The distance from the origin is \\(\sqrt{x^2 + y^2}\\). You may return the answer in any order.

**Constraints:**
- \\( 1 \le k \le \text{points.length} \le 10^4 \\)
- \\( -10^4 \le x_i, y_i \le 10^4 \\)

### Examples

**Example 1:**
```text
Input: points = [[1,3],[-2,2]], k = 1
Output: [[-2,2]]
Explanation: dist([1,3]) = sqrt(10), dist([-2,2]) = sqrt(8). Closest is [-2,2].
```

**Example 2:**
```text
Input: points = [[3,3],[5,-1],[-2,4]], k = 2
Output: [[3,3],[-2,4]]
```

---

### Solution

Maintain a max-heap of size `k` keyed by squared distance (no need to take the square root). Push each point; if the heap exceeds `k`, pop the farthest. The remaining `k` entries are the answer.

Comparing squared distances avoids floating-point arithmetic entirely.

```rust,ignore
use std::collections::BinaryHeap;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    	let k = k as usize;
        let mut heap: BinaryHeap<(i32, Vec<i32>)> = BinaryHeap::new();

        for point in points {
        	let x = point[0];
        	let y = point[1];

        	let dist_sq = x * x + y * y;

        	heap.push((dist_sq, point));

        	if heap.len() > k {
        		heap.pop();
        	}
        }

        heap.into_iter().map(|(_, point)| point).collect()
    }
}
```
