# Valid Parentheses

[LeetCode #20](https://leetcode.com/problems/valid-parentheses/description/)

---

### Statement

Given a string `s` containing just the characters `'('`, `')'`, `'{'`, `'}'`, `'['` and `']'`, determine if the input string is valid.

An input string is valid if:
1. Open brackets must be closed by the same type of brackets.
2. Open brackets must be closed in the correct order.
3. Every close bracket has a corresponding open bracket of the same type.

**Constraints:**
- \\( 1 \le \text{s.length} \le 10^4 \\)
- `s` consists of parentheses only `'()[]{}'`.

### Examples

**Example 1:**
```text
Input: s = "()"
Output: true
```

**Example 2:**
```text
Input: s = "()[]{}"
Output: true
```

**Example 3:**
```text
Input: s = "(]"
Output: false
```

---

### Solution

#### Initial approach:

Push open brackets onto the stack. For each closing bracket, check if the stack's top matches before popping; otherwise push the unmatched closer (so the stack is non-empty at the end).

```rust,ignore
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for ch in s.chars() {
            if ch == '(' || ch == '[' || ch == '{' {
                stack.push(ch);
                continue;
            }

            if ch == ']' && !stack.is_empty() && *stack.last().unwrap() == '[' {
                stack.pop();
            } else if ch == '}' && !stack.is_empty() && *stack.last().unwrap() == '{' {
                stack.pop();
            } else if ch == ')' && !stack.is_empty() && *stack.last().unwrap() == '(' {
                stack.pop();
            } else {
                stack.push(ch);
            }
        }

        stack.is_empty()
    }
}
```

#### Cleaner approach (push expected closer):

Instead of storing the opener and checking against it, push the *expected closing bracket* when an opener is seen. Then for any closer, just verify it matches the top with a single `pop()` call.

```rust,ignore
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for ch in s.chars() {
            match ch {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                c => {
                    if stack.pop() != Some(c) {
                        return false;
                    }
                }
            }
        }

        stack.is_empty()
    }
}
```
