# Rotting Oranges

[LeetCode #994](https://leetcode.com/problems/rotting-oranges/description/)

---

### Statement

You are given an `m x n` grid where each cell is one of:
- `0` — empty
- `1` — fresh orange
- `2` — rotten orange

Every minute, any fresh orange **4-directionally adjacent** to a rotten orange becomes rotten. Return the minimum number of minutes until no fresh orange remains, or `-1` if that is impossible.

**Constraints:**
- `m == grid.length`, `n == grid[i].length`
- \\( 1 \le m, n \le 10 \\)
- `grid[i][j]` is `0`, `1`, or `2`.

### Examples

**Example 1:**
```text
Input: grid = [[2,1,1],[1,1,0],[0,1,1]]
Output: 4
```

**Example 2:**
```text
Input: grid = [[2,1,1],[0,1,1],[1,0,1]]
Output: -1
Explanation: The orange in the bottom-left is isolated.
```

**Example 3:**
```text
Input: grid = [[0,2]]
Output: 0
Explanation: No fresh oranges exist.
```

---

### Solution

Multi-source BFS from all initially rotten oranges simultaneously. Seed the queue with every `2` cell, then spread to adjacent `1`s, tracking the elapsed time. After BFS, any remaining `1` means an unreachable orange — return `-1`.

```rust,ignore
use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        let m = grid.len() as i32;
        let n = grid[0].len() as i32;

        let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();

        for i in 0..m {
        	for j in 0..n {
        		if grid[i as usize][j as usize] == 2 {
        			queue.push_back((i, j, 0));
        		}
        	}
        }

       	let mut max_time = 0;

       	while let Some((i, j, time)) = queue.pop_front() {
       		max_time = max_time.max(time);

       		for (dx, dy) in DIRECTIONS {
       			let (ni, nj) = (i + dx, j + dy);

       			if ni >= 0 && ni < m && nj >= 0 && nj < n && grid[ni as usize][nj as usize] == 1 {
       				grid[ni as usize][nj as usize] = 2;
       				queue.push_back((ni, nj, time + 1));
       			}
       		}
       	}

       	if grid.iter().any(|row| row.contains(&1)) {
            return -1;
        }

        max_time
    }
}
```
