# Linked List Cycle

[LeetCode #141](https://leetcode.com/problems/linked-list-cycle/description/)

---

### Statement

Given `head`, the head of a linked list, determine if the linked list has a cycle in it.

There is a cycle in a linked list if there is some node in the list that can be reached again by continuously following the `next` pointer.

Return `true` if there is a cycle in the linked list. Otherwise, return `false`.

**Constraints:**
- The number of nodes in the list is in the range \\([0, 10^4]\\).
- \\( -10^5 \le \text{Node.val} \le 10^5 \\)
- `pos` is `-1` or a valid index in the linked list.

### Examples

**Example 1:**
```text
Input: head = [3,2,0,-4], pos = 1
Output: true
Explanation: There is a cycle in the linked list, where the tail connects to the 1st node (0-indexed).
```

**Example 2:**
```text
Input: head = [1,2], pos = 0
Output: true
Explanation: There is a cycle in the linked list, where the tail connects to the 0th node.
```

**Example 3:**
```text
Input: head = [1], pos = -1
Output: false
Explanation: There is no cycle in the linked list.
```

---

### Solution

**This problem is fundamentally incompatible with Rust's ownership model.**

LeetCode's node definition uses `Box<T>`, which enforces single ownership. A cycle requires a node to be reachable from two places simultaneously — two owners of the same node — which is illegal in safe Rust. LeetCode therefore does not offer this problem in Rust.

```text
3 → 2 → 0 → -4
        ↑_____↑
     (tail points back to node 2 — TWO owners of node 2)
```

To represent a cycle in Rust you would need `Rc<RefCell<ListNode>>` (reference counted, shared ownership) or `unsafe` raw pointers, neither of which matches LeetCode's simple `Box`-based definition.

**JavaScript solution (Floyd's cycle detection — fast and slow pointers):**

```js
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
