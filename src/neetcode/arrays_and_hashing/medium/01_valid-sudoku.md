# Valid Sudoku

[LeetCode #36](https://leetcode.com/problems/valid-sudoku/description/)

---

### Statement

Determine if a `9 x 9` Sudoku board is valid. Only filled cells need to be validated according to these rules:

1. Each row must contain the digits `1–9` without repetition.
2. Each column must contain the digits `1–9` without repetition.
3. Each of the nine `3 x 3` sub-boxes must contain the digits `1–9` without repetition.

Empty cells are denoted by `'.'`.

**Constraints:**
- `board.length == 9`, `board[i].length == 9`
- `board[i][j]` is a digit `'1'–'9'` or `'.'`.

### Examples

**Example 1:**
```text
Input: board =
[["5","3",".",".","7",".",".",".","."]
,["6",".",".","1","9","5",".",".","."]
,[".","9","8",".",".",".",".","6","."]
,["8",".",".",".","6",".",".",".","3"]
,["4",".",".","8",".","3",".",".","1"]
,["7",".",".",".","2",".",".",".","6"]
,[".","6",".",".",".",".","2","8","."]
,[".",".",".","4","1","9",".",".","5"]
,[".",".",".",".","8",".",".","7","9"]]
Output: true
```

**Example 2:**
```text
Input: board with an 8 repeated in the top-left 3x3 box
Output: false
```

---

### Solution

#### Three-pass approach:

Check rows, then columns, then each 3×3 box separately. For each unit, track seen digits with a `[0; 10]` array.

```rust,ignore
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        const SIZE: usize = 9;

        for i in 0..SIZE {
        	let mut digits = [0; 10];
        	for j in 0..SIZE {
        		let char = board[i][j];

        		if char != '.' {
        			let mut digit = (char as u8 - b'0') as usize;
        			if digits[digit] == 1 {
        				return false;
        			}

        			digits[digit] += 1;
        		}
        	}
        }

        for i in 0..SIZE {
        	let mut digits = [0; 10];
        	for j in 0..SIZE {
        		let char = board[j][i];

        		if char != '.' {
        			let mut digit = (char as u8 - b'0') as usize;
        			if digits[digit] == 1 {
        				return false;
        			}

        			digits[digit] += 1;
        		}
        	}
        }


        for i in (0..SIZE).step_by(3) {
        	for j in (0..SIZE).step_by(3) {
        		let mut digits = [0; 10];

        		for x in i..i+3 {
        			for y in j..j+3 {
        				let char = board[x][y];
        				if char != '.' {
        					let mut digit = (char as u8 - b'0') as usize;
        					if digits[digit] == 1 {
        						return false;
        					}
        					digits[digit] += 1;
        				}
        			}
        		}
        	}
        }

        true
    }
}
```

#### Single-pass approach:

One iteration over all cells. Compute the box index as `(r/3)*3 + (c/3)` and track seen digits for rows, columns, and boxes simultaneously using three `[[bool; 9]; 9]` arrays. Uses `'1'`-based indexing (`d = digit - '1'`) for zero-indexed slots.

```rust,ignore
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows  = [[false; 9]; 9];
        let mut cols  = [[false; 9]; 9];
        let mut boxes = [[false; 9]; 9];

        for r in 0..9 {
            for c in 0..9 {
                let ch = board[r][c];
                if ch == '.' { continue; }

                let d = (ch as u8 - b'1') as usize; // 0-indexed: '1'->0, '9'->8
                let b = (r / 3) * 3 + (c / 3);      // box index 0..9

                if rows[r][d] || cols[c][d] || boxes[b][d] {
                    return false;
                }

                rows[r][d]  = true;
                cols[c][d]  = true;
                boxes[b][d] = true;
            }
        }

        true
    }
}
```
