# Design Circular Queue

[LeetCode #622](https://leetcode.com/problems/design-circular-queue/description/)

---

### Statement

Design a circular queue of fixed capacity `k`. A circular queue reuses the freed space after dequeue operations.

Implement `MyCircularQueue`:
- `MyCircularQueue(k)` — initializes the queue with capacity `k`.
- `boolean enQueue(int value)` — inserts an element; returns `false` if full.
- `boolean deQueue()` — deletes the front element; returns `false` if empty.
- `int Front()` — returns the front element, or `-1` if empty.
- `int Rear()` — returns the last element, or `-1` if empty.
- `boolean isEmpty()` — checks whether empty.
- `boolean isFull()` — checks whether full.

**Constraints:**
- \\( 1 \le k \le 1000 \\)
- \\( 0 \le \text{value} \le 1000 \\)
- At most \\( 3000 \\) calls to the above methods.

### Examples

**Example 1:**
```text
Input:  ["MyCircularQueue","enQueue","enQueue","enQueue","enQueue","Rear",
         "isFull","deQueue","enQueue","Rear"]
        [[3],[1],[2],[3],[4],[],[],[],[4],[]]
Output: [null,true,true,true,false,3,true,true,true,4]
```

---

### Solution

*Not yet implemented.*
