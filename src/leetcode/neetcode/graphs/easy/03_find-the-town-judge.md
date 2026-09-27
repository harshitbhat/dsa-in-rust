# Find the Town Judge

[LeetCode #997](https://leetcode.com/problems/find-the-town-judge/description/)

---

### Statement

In a town of `n` people (labeled `1` to `n`), the town judge (if they exist):
1. Trusts nobody.
2. Is trusted by everybody else.
3. Is the only such person.

Given `trust` where `trust[i] = [a, b]` means person `a` trusts person `b`, return the label of the town judge, or `-1` if they don't exist.

**Constraints:**
- \\( 1 \le n \le 1000 \\)
- \\( 0 \le \text{trust.length} \le 10^4 \\)
- `trust[i].length == 2`, `a != b`, all pairs unique.
- \\( 1 \le a, b \le n \\)

### Examples

**Example 1:**
```text
Input: n = 2, trust = [[1,2]]
Output: 2
```

**Example 2:**
```text
Input: n = 3, trust = [[1,3],[2,3]]
Output: 3
```

**Example 3:**
```text
Input: n = 3, trust = [[1,3],[2,3],[3,1]]
Output: -1
```

---

### Solution

#### Initial approach (adjacency list):

Build an adjacency list. The judge is the only person with zero out-edges who is pointed to by everyone else.

```rust,ignore
use std::collections::HashMap;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

        for t in trust {
            let a = t[0];
            let b = t[1];
        	graph.entry(a).or_insert_with(Vec::new).push(b);
        }

        for i in 1..=n {
            graph.entry(i).or_insert(Vec::new());
        }

        let mut zero_entries = 0;
        let mut judge_key = -1;

        for (k, v) in &graph {
        	if v.len() == 0 {
        		zero_entries += 1;
        		judge_key = *k;
        	}
        }

        if zero_entries != 1 {
        	return -1;
        } else {
        	for (k, v) in &graph {
        		if *k != judge_key {
        			if !v.contains(&judge_key) {
        				return -1;
        			}
        		}
        	}
        	return judge_key;
        }
    }
}
```

#### Better approach (in/out degree):

Track two counts per person: how many they trust (out-degree) and how many trust them (in-degree). The judge has out-degree `0` and in-degree `n - 1`.

```rust,ignore
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut in_degree = vec![0i32; n + 1];
        let mut out_degree = vec![0i32; n + 1];

        for t in &trust {
            out_degree[t[0] as usize] += 1;
            in_degree[t[1] as usize] += 1;
        }

        for i in 1..=n {
            if out_degree[i] == 0 && in_degree[i] == n as i32 - 1 {
                return i as i32;
            }
        }

        -1
    }
}
```
