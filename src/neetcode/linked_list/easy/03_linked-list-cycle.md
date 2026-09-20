

### Solution:

**This problem is fundamentally incompatible with Rust's ownership model.**

The reason is the `ListNode` definition itself:

```
// LeetCode's node definition
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>   // ← Box means single owner
}
```

`Box<T>` enforces **single ownership** — every node has exactly one owner. A cycle requires a node to be reachable from *two places* (a node pointing back to an earlier node), which means **two owners for the same node**. That's illegal in safe Rust.

```
3 → 2 → 0 → -4
        ↑_____↑
     (tail points back to node 2 — TWO owners of node 2)
```

For LeetCode to even *construct* the test input in Rust, they'd need to use either:
- `Rc<RefCell<ListNode>>` — reference counted, shared ownership
- `unsafe` raw pointers

Both are complex and not what LeetCode's simple `Box`-based `ListNode` supports. So **LeetCode simply doesn't offer this problem in Rust** because they can't represent the input type safely with their existing node definition.

---

**JS Solution**

```js
/**
 * Definition for singly-linked list.
 * function ListNode(val) {
 *     this.val = val;
 *     this.next = null;
 * }
 */

/**
 * @param {ListNode} head
 * @return {boolean}
 */
var hasCycle = function (head) {
  let slow = head;
  let fast = head;

  while (fast && fast.next) {
    slow = slow.next;
    fast = fast.next.next;

    if (slow === fast) {
      return true;
    }
  }
  return false;
};
```