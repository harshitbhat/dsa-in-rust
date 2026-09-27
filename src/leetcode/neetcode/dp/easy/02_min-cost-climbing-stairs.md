# Min Cost Climbing Stairs

[LeetCode #746](https://leetcode.com/problems/min-cost-climbing-stairs/description/)

---

### Statement

Given an integer array `cost` where `cost[i]` is the cost of the `i`th step, you can start from index `0` or `1`. From each step you can climb one or two steps. Return the minimum cost to reach the top (past the last index).

**Constraints:**
- \\( 2 \le \text{cost.length} \le 1000 \\)
- \\( 0 \le \text{cost}[i] \le 999 \\)

### Examples

**Example 1:**
```text
Input: cost = [10,15,20]
Output: 15
Explanation: Start at index 1, pay 15, jump 2 steps to the top.
```

**Example 2:**
```text
Input: cost = [1,100,1,1,1,100,1,1,100,1]
Output: 6
```

---

### Solution

`dp[i]` = minimum total cost to leave step `i`. The cost to leave step `i` is `cost[i]` plus the cheaper of the two previous departure costs. The answer is the minimum of the last two entries (since you can reach the top from either of them).

```rust,ignore
use std::cmp::min;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        if cost.len() <= 2 {
        	return min(cost[0], cost[1]);
        }

        let n = cost.len();
        let mut dp = vec![0; n];

        dp[0] = cost[0];
        dp[1] = cost[1];

        for i in 2..n {
        	dp[i] = cost[i] + min(dp[i - 1], dp[i - 2]);
        }

        min(dp[n - 1], dp[n - 2])
    }
}
```
