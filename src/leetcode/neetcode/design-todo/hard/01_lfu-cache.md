# LFU Cache

[LeetCode #460](https://leetcode.com/problems/lfu-cache/description/)

---

### Statement

Design and implement a **Least Frequently Used (LFU)** cache.

Implement `LFUCache`:
- `LFUCache(int capacity)` — initializes the cache with `capacity`.
- `int get(int key)` — returns the value if present (and increments its frequency), else `-1`.
- `void put(int key, int value)` — inserts or updates the value. If the cache is at capacity, evict the least frequently used key. On a tie, evict the **least recently used** among those.

Both operations must run in \\(\mathcal{O}(1)\\) average time.

**Constraints:**
- \\( 1 \le \text{capacity} \le 10^4 \\)
- \\( 0 \le \text{key} \le 10^5 \\)
- \\( 0 \le \text{value} \le 10^9 \\)
- At most \\( 2 \times 10^5 \\) calls to `get` and `put`.

### Examples

**Example 1:**
```text
Input:  ["LFUCache","put","put","get","put","get","get","put","get","get","get"]
        [[2],[1,1],[2,2],[1],[3,3],[2],[3],[4,4],[1],[3],[4]]
Output: [null,null,null,1,null,-1,3,null,-1,3,4]
```

---

### Solution

*Not yet implemented.*
