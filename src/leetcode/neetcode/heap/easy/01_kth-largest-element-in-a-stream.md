# Kth Largest Element in a Stream

[LeetCode #703](https://leetcode.com/problems/kth-largest-element-in-a-stream/description/)

---

### Statement

Design a class to find the `k`th largest element in a stream. Note that it is the `k`th largest element in sorted order, not the `k`th distinct element.

Implement the `KthLargest` class:
- `KthLargest(int k, int[] nums)` — initializes the object with integer `k` and the stream `nums`.
- `int add(int val)` — appends `val` to the stream and returns the `k`th largest element.

**Constraints:**
- \\( 0 \le \text{nums.length} \le 10^4 \\)
- \\( 1 \le k \le 10^4 \\)
- \\( -10^4 \le \text{nums}[i] \le 10^4 \\)
- \\( -10^4 \le \text{val} \le 10^4 \\)
- At most \\( 10^4 \\) calls will be made to `add`.
- It is guaranteed that there will be at least `k` elements in the array when searching for the `k`th element.

### Examples

**Example 1:**
```text
Input:
  ["KthLargest", "add", "add", "add", "add", "add"]
  [[3, [4, 5, 8, 2]], [3], [5], [10], [9], [4]]
Output:
  [null, 4, 5, 5, 8, 8]
```

---

### Solution

Maintain a min-heap of size `k`. The smallest element in the heap is always the `k`th largest seen so far. On each `add`, push the new value; if the heap exceeds size `k`, pop the minimum. The top of the heap is the answer.

Using `Reverse` from `std::cmp` turns Rust's max-heap (`BinaryHeap`) into a min-heap.

```rust,ignore
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct KthLargest {
	k: usize,
	heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
	fn new(k: i32, nums: Vec<i32>) -> Self {
		let mut obj = KthLargest {
			k: k as usize,
			heap: BinaryHeap::with_capacity(k as usize + 1),
		};

		for num in nums {
			obj.add(num);
		}

		obj
	}

	fn add(&mut self, val: i32) -> i32 {
		self.heap.push(Reverse(val));

		if self.heap.len() > self.k {
			self.heap.pop();
		}

		self.heap.peek().unwrap().0 // .0 unwraps the Reverse
	}
}
```
