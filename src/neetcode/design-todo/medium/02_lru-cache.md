# LRU Cache

[LeetCode #146](https://leetcode.com/problems/lru-cache/description/)

---

### Statement

Design a data structure that follows the **Least Recently Used (LRU)** cache eviction policy.

Implement `LRUCache`:
- `LRUCache(int capacity)` — initializes the cache with `capacity`.
- `int get(int key)` — returns the value if present (and marks it recently used), else `-1`.
- `void put(int key, int value)` — inserts or updates the value. If the cache is full, evict the least recently used key first.

Both operations must run in \\(\mathcal{O}(1)\\) average time.

**Constraints:**
- \\( 1 \le \text{capacity} \le 3000 \\)
- \\( 0 \le \text{key} \le 10^4 \\)
- \\( 0 \le \text{value} \le 10^5 \\)
- At most \\( 2 \times 10^5 \\) calls to `get` and `put`.

### Examples

**Example 1:**
```text
Input:  ["LRUCache","put","put","get","put","get","put","get","get","get"]
        [[2],[1,1],[2,2],[1],[3,3],[2],[4,4],[1],[3],[4]]
Output: [null,null,null,1,null,-1,null,-1,3,4]
```

---

### Solution

*Not yet implemented.*
