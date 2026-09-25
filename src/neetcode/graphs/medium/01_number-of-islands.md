# Number of Islands

[LeetCode #200](https://leetcode.com/problems/number-of-islands/description/)

---

### Statement

Given an `m x n` 2D binary grid of `'1'`s (land) and `'0'`s (water), return the number of islands. An island is surrounded by water and formed by connecting adjacent land cells horizontally or vertically.

**Constraints:**
- `m == grid.length`, `n == grid[i].length`
- \\( 1 \le m, n \le 300 \\)
- `grid[i][j]` is `'0'` or `'1'`.

### Examples

**Example 1:**
```text
Input: grid = [
  ["1","1","1","1","0"],
  ["1","1","0","1","0"],
  ["1","1","0","0","0"],
  ["0","0","0","0","0"]
]
Output: 1
```

**Example 2:**
```text
Input: grid = [
  ["1","1","0","0","0"],
  ["1","1","0","0","0"],
  ["0","0","1","0","0"],
  ["0","0","0","1","1"]
]
Output: 3
```

---

### Solution

DFS flood-fill. Scan the grid; whenever an unvisited `'1'` is found, increment the island count and DFS to mark all connected land cells as visited.

```rust,ignore
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut seen = vec![vec![false; n]; m];
        let mut islands = 0;

        for i in 0..m {
        	for j in 0..n {
        		if grid[i][j] == '1' && !seen[i][j] {
        			Self::dfs(&grid, &mut seen, i, j, m, n);
        			islands += 1;
        		}
        	}
        }

        islands
    }

    pub fn dfs(
    	grid: &Vec<Vec<char>>,
    	seen: &mut Vec<Vec<bool>>,
    	i: usize,
    	j: usize,
    	m: usize,
    	n: usize,
    ) {
    	if seen[i][j] {
    		return;
    	}

    	seen[i][j] = true;

    	if i >= 1 && !seen[i - 1][j] && grid[i - 1][j] == '1' {
    		Self::dfs(grid, seen, i - 1, j, m, n);
    	}

    	if j < n - 1 && !seen[i][j + 1] && grid[i][j + 1] == '1' {
    		Self::dfs(grid, seen, i, j + 1, m, n);
    	}

    	if i < m - 1 && !seen[i + 1][j] && grid[i + 1][j] == '1' {
    		Self::dfs(grid, seen, i + 1, j, m, n);
    	}

    	if j >= 1 && !seen[i][j - 1] && grid[i][j - 1] == '1' {
    		Self::dfs(grid, seen, i, j - 1, m, n);
    	}
    }
}
```
