# Merge Two Sorted Lists

[LeetCode #21](https://leetcode.com/problems/merge-two-sorted-lists/description/)

---

### Statement

You are given the heads of two sorted linked lists `list1` and `list2`.

Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists. Return the head of the merged linked list.

**Constraints:**
- The number of nodes in both lists is in the range \\([0, 50]\\).
- \\( -100 \le \text{Node.val} \le 100 \\)
- Both `list1` and `list2` are sorted in non-decreasing order.

### Examples

**Example 1:**
```text
Input: list1 = [1,2,4], list2 = [1,3,4]
Output: [1,1,2,3,4,4]
```

**Example 2:**
```text
Input: list1 = [], list2 = []
Output: []
```

**Example 3:**
```text
Input: list1 = [], list2 = [0]
Output: [0]
```

---

### Solution

I had the mental framework in mind but couldn't express it in Rust, so I looked for help with the translation.

**The JS → Rust translation, concept by concept:**

```text
JS                          Rust
──────────────────────────────────────────────────────
new ListNode(-1)            Box::new(ListNode::new(-1))
let prev = prehead          let mut prev = &mut prehead   // borrow, not copy
l1 && l2                    l1.is_some() && l2.is_some()
l1.val                      l1.as_ref().unwrap().val      // peek without moving
prev.next = l1              prev.next = l1                // moves ownership in
l1 = l1.next               l1 = prev.next.as_mut()
                                 .unwrap().next.take()    // .take() is the key
prev = prev.next            prev = prev.next.as_mut().unwrap()
prehead.next                prehead.next                  // moves out of dummy
```

**Two Rust-specific patterns to internalize:**

**1. `.as_ref().unwrap().val` — peeking without consuming**
```rust,ignore
// You can't do: l1.unwrap().val  ← this MOVES l1 out, destroying it
// Instead:
l1.as_ref().unwrap().val   // borrows temporarily, l1 still alive after
```

**2. `.take()` — the "advance pointer" trick**
```rust,ignore
// In JS:  l1 = l1.next   ← just reassign the variable
// In Rust, after prev.next = l1, you no longer own l1.
// The node is now inside prev.next, so to get l1.next you do:
l1 = prev.next.as_mut().unwrap().next.take();
//                                    ^^^^^^
//                         moves .next out, leaves None behind
```

#### Recursive approach (much cleaner in Rust):

Maps almost 1:1 to how you'd think about the problem — "take the smaller head, recurse on the rest." No pointer juggling.

```rust,ignore
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, r) => r,
            (l, None) => l,
            (Some(mut l), Some(mut r)) => {
                if l.val <= r.val {
                    l.next = Self::merge_two_lists(l.next.take(), Some(r));
                    Some(l)
                } else {
                    r.next = Self::merge_two_lists(Some(l), r.next.take());
                    Some(r)
                }
            }
        }
    }
}
```
