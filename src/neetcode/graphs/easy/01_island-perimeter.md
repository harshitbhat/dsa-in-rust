# Island Perimeter

[LeetCode #463](https://leetcode.com/problems/island-perimeter/description/)

---

### Statement

You are given `row x col` grid representing a map where `grid[i][j] = 1` represents land and `grid[i][j] = 0` represents water.

Grid cells are connected horizontally/vertically (not diagonally). There is exactly one island with no lakes. Determine the perimeter of the island.

**Constraints:**
- `row == grid.length`
- `col == grid[i].length`
- \\( 1 \le \text{row, col} \le 100 \\)
- `grid[i][j]` is `0` or `1`
- There is exactly one island in `grid`.

### Examples

**Example 1:**
```text
Input: grid = [[0,1,0,0],[1,1,1,0],[0,1,0,0],[1,1,0,0]]
Output: 16
```

**Example 2:**
```text
Input: grid = [[1]]
Output: 4
```

**Example 3:**
```text
Input: grid = [[1,0]]
Output: 4
```

---

### Solution

Each land cell contributes 4 to the perimeter. For every shared edge between two adjacent land cells, subtract 2 (one side from each cell). Only check the top and left neighbours to avoid double-counting: each interior edge is handled exactly once.

```rust,ignore
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut perimeter = 0;
        let mut rows = grid.len();
        let mut cols = grid[0].len();

        for i in 0..cols {
        	if grid[0][i] == 1 {
        		perimeter += 4;
        	}
        }

        for i in 0..rows {
        	if grid[i][0] == 1 {
        		perimeter += 4;
        	}
        }

        for i in 1..rows {
        	for j in 1..cols {
        		if grid[i][j] == 1 {
        			perimeter += 4;
        			if grid[i - 1][j] == 1 {
        				perimeter -= 2;
        			}
        			if grid[i][j - 1] == 1 {
        				perimeter -= 2;
        			}
        		}
        	}
        }

        perimeter
    }
}
```
