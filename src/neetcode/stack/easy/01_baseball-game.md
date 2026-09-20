# Baseball Game

[LeetCode #682](https://leetcode.com/problems/baseball-game/description/)

---

### Statement

You are keeping the scores for a baseball game with strange rules. At the beginning of the game, you start with an empty record.

You are given a list of strings `operations`, where `operations[i]` is the `i`th operation you must apply to the record and is one of the following:

- An integer `x` — record a new score of `x`.
- `"+"` — record a new score that is the sum of the previous two scores.
- `"D"` — record a new score that is double the previous score.
- `"C"` — invalidate the previous score, removing it from the record.

Return the sum of all the scores on the record after applying all the operations.

**Constraints:**
- \\( 1 \le \text{operations.length} \le 1000 \\)
- `operations[i]` is `"C"`, `"D"`, `"+"`, or a string representing an integer in the range \\([-3 \times 10^4, 3 \times 10^4]\\).
- For `"+"`, there will always be at least two previous scores on the record.
- For `"D"` and `"C"`, there will always be at least one previous score on the record.

### Examples

**Example 1:**
```text
Input: operations = ["5","2","C","D","+"]
Output: 30
Explanation:
"5"  → record: [5]
"2"  → record: [5, 2]
"C"  → record: [5]
"D"  → record: [5, 10]
"+"  → record: [5, 10, 15]
Sum = 5 + 10 + 15 = 30
```

**Example 2:**
```text
Input: operations = ["5","-2","4","C","D","9","+","+"]
Output: 27
```

---

### Solution

Simulate with a stack. Match on each operation string and manipulate the top of the stack accordingly using helper methods to keep the match arms clean.

```rust,ignore
impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut result: Vec<i32> = Vec::new();

        for op in operations {
            match op.as_str() {
                "+" => {
                    result.push(Self::get_top_two_sum(&result));
                },
                "D" => {
                    result.push(2 * Self::get_top(&result));
                },
                "C" => {
                    result.pop();
                },
                _ => {
                    result.push(op.parse().unwrap());
                }
            }
        }

        result.iter().sum()
    }

    pub fn get_top(array: &Vec<i32>) -> i32 {
        let n = array.len();
        array[n - 1]
    }

    pub fn get_top_two_sum(array: &Vec<i32>) -> i32 {
        let n = array.len();
        array[n - 1] + array[n - 2]
    }
}
```
