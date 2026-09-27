# Design HashSet

[LeetCode #705](https://leetcode.com/problems/design-hashset/description/)

---

### Statement

Design a HashSet **without** using built-in hash table libraries.

Implement `MyHashSet`:
- `MyHashSet()` — initializes an empty set.
- `void add(int key)` — inserts `key` into the set.
- `void remove(int key)` — removes `key` if present.
- `bool contains(int key)` — returns whether `key` is in the set.

**Constraints:**
- \\( 0 \le \text{key} \le 10^6 \\)
- At most \\( 10^4 \\) calls to `add`, `remove`, and `contains`.

### Examples

**Example 1:**
```text
Input:  ["MyHashSet","add","add","contains","contains","add","contains","remove","contains"]
        [[],[1],[2],[1],[3],[2],[2],[2],[2]]
Output: [null,null,null,true,false,null,true,null,false]
```

---

### Solution

```rust,ignore
const MAX: usize = 1000001;

struct MyHashSet {
        set: Vec<bool>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {

    fn new() -> Self {
        Self {
                set: vec![false; MAX],
        }
    }
    
    fn add(&mut self, key: i32) {
        let key = key as usize;
        self.set[key] = true;
    }
    
    fn remove(&mut self, key: i32) {
        let key = key as usize;
        self.set[key] = false;
    }
    
    fn contains(&self, key: i32) -> bool {
        let key = key as usize;
        self.set[key] == true
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */
```

Not best, the current approach is the "direct addressing" approach — allocating a giant boolean array indexed by key value. It works and is O(1) for all ops, but wastes ~1MB of memory upfront regardless of how many keys you actually store.

The proper approach: Hash Table with Separate Chaining

The standard interview-expected solution uses:

- A fixed array of buckets (e.g., 1000 buckets)
- Each bucket holds a linked list of keys that hash to it
- Hash function: `key % num_buckets`

```rust,ignore

const BUCKET_COUNT: usize = 1031;

struct MyHashSet {
        buckets: Vec<Vec<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {

    fn new() -> Self {
        Self {
                buckets: vec![Vec::new(); BUCKET_COUNT],
        }
    }

    fn hash(&self, key: i32) -> usize {
        key as usize % BUCKET_COUNT
    }
    
    fn add(&mut self, key: i32) {
        let h = self.hash(key);

        if !self.buckets[h].contains(&key) {
                self.buckets[h].push(key);
        }
    }
    
    fn remove(&mut self, key: i32) {
        let h = self.hash(key);

        self.buckets[h].retain(|&k| k != key);
    }
    
    fn contains(&self, key: i32) -> bool {
        let h = self.hash(key);
        self.buckets[h].contains(&key)
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */
```