# Reverse Linked List

[LeetCode #206](https://leetcode.com/problems/reverse-linked-list/description/)

---

### Statement

Given the `head` of a singly linked list, reverse the list and return the reversed list.

**Constraints:**
- The number of nodes in the list is in the range \\([0, 5000]\\).
- \\( -5000 \le \text{Node.val} \le 5000 \\)

### Examples

**Example 1:**
```text
Input: head = [1,2,3,4,5]
Output: [5,4,3,2,1]
```

**Example 2:**
```text
Input: head = [1,2]
Output: [2,1]
```

**Example 3:**
```text
Input: head = []
Output: []
```

---

### Solution

Iterative approach. Walk the list keeping a `prev` handle. At each node, take ownership of the current node out of the list with `.take()`, point its `next` back to `prev`, then advance both pointers forward.

`.take()` is the key Rust idiom here — it moves the value out of an `Option`, leaving `None` in its place, which avoids the borrow-checker conflict of holding two mutable references to the same list at once.

```rust,ignore
impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;

        while let Some(mut node) = head {
            head = node.next.take();
            node.next = prev;
            prev = Some(node);
        }

        prev
    }
}
```
