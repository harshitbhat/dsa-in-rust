# Lemonade Change

[LeetCode #860](https://leetcode.com/problems/lemonade-change/description/)

---

### Statement

At a lemonade stand, each lemonade costs `$5`. Customers pay with `$5`, `$10`, or `$20` bills, one at a time. You start with no change. Return `true` if you can give every customer exact change, `false` otherwise.

**Constraints:**
- \\( 1 \le \text{bills.length} \le 10^5 \\)
- `bills[i]` is `5`, `10`, or `20`.

### Examples

**Example 1:**
```text
Input: bills = [5,5,5,10,20]
Output: true
```

**Example 2:**
```text
Input: bills = [5,5,10,10,20]
Output: false
```

---

### Solution

Greedily prefer to give `$10 + $5` change for a `$20` bill before falling back to `3 × $5`, since `$5` bills are more versatile.

#### Initial approach:

```rust,ignore
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut change_5 = 0;
        let mut change_10 = 0;
        let mut change_20 = 0;

        for bill in bills {
        	if bill == 5 {
        		change_5 += 1;
        		continue;
        	} else if bill == 10 {
        		if change_5 < 1 {
        			return false;
        		}

        		change_5 -= 1;
        		change_10 += 1;
        	} else {
        		if change_10 > 0 && change_5 > 0 {
        			change_20 += 1;
        			change_10 -= 1;
        			change_5 -= 1;
        		} else if change_5 > 2 {
        			change_5 -= 3;
        			change_20 += 1;
        		} else {
        			return false;
        		}
        	}
        }

        true
    }
}
```

#### Cleaner with `match`:

```rust,ignore
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let (mut five, mut ten) = (0, 0);

        for bill in bills {
        	match bill {
        		5 => five += 1,
        		10 => {
        			if five == 0 { return false; }
        			five -= 1;
        			ten += 1;
        		}
        		_ => {
        			if ten > 0 && five > 0 {
        				ten -= 1;
        				five -= 1;
        			} else if five >= 3 {
        				five -= 3;
        			} else {
        				return false;
        			}
        		}
        	}
        }
        true
    }
}
```
