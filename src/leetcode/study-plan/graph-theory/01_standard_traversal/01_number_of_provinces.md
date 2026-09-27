# Number of Provinces

[LeetCode #547](https://leetcode.com/problems/number-of-provinces/description/)

---

### Statement

There are `n` cities. Some of them are connected, while some are not. If city `a` is connected directly with city `b`, and city `b` is connected directly with city `c`, then city `a` is connected indirectly with city `c`.

A **province** is a group of directly or indirectly connected cities and no other cities outside of the group.

You are given an `n x n` matrix `isConnected` where `isConnected[i][j] = 1` if the `i`th city and the `j`th city are directly connected, and `isConnected[i][j] = 0` otherwise.

Return the total number of provinces.

**Constraints:**
- \\( 1 \le n \le 200 \\)
- `n == isConnected.length`
- `n == isConnected[i].length`
- `isConnected[i][j]` is `1` or `0`
- `isConnected[i][i] == 1`
- `isConnected[i][j] == isConnected[j][i]`

### Examples

**Example 1:**
```text
Input:  isConnected = [[1,1,0],[1,1,0],[0,0,1]]
Output: 2
```

**Example 2:**
```text
Input:  isConnected = [[1,0,0],[0,1,0],[0,0,1]]
Output: 3
```

---

### Solution

```rust,ignore
impl Solution {
    pub fn find_circle_num(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let mut seen = vec![false; n];

        let mut count = 0;

        for i in 0..n {
            if !seen[i] {
                count += 1;
                Self::dfs(&graph, i, n, &mut seen);
            }
        }

        count
    }

    pub fn dfs(graph: &Vec<Vec<i32>>, src: usize, n: usize, seen: &mut Vec<bool>) {
        if seen[src] {
            return;
        }

        seen[src] = true;

        for i in 0..n {
            if graph[src][i] == 1 && !seen[i] {
                Self::dfs(graph, i, n, seen);
            }
        }
    }
}
```

Time: O(n²) — every cell in the adjacency matrix is visited once. Space: O(n) for the `seen` array and call stack.
