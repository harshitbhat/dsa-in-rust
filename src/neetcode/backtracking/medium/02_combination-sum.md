# Combination Sum

[LeetCode #39](https://leetcode.com/problems/combination-sum/description/)

---

### Statement

Given an array of **distinct** integers `candidates` and a `target` integer, return a list of all unique combinations where the chosen numbers sum to `target`. The same number may be chosen **unlimited** times. Return the combinations in any order.

**Constraints:**
- \\( 1 \le \text{candidates.length} \le 30 \\)
- \\( 2 \le \text{candidates}[i] \le 40 \\)
- All values of `candidates` are distinct.
- \\( 1 \le \text{target} \le 40 \\)

### Examples

**Example 1:**
```text
Input: candidates = [2,3,6,7], target = 7
Output: [[2,2,3],[7]]
```

**Example 2:**
```text
Input: candidates = [2,3,5], target = 8
Output: [[2,2,2,2],[2,3,3],[3,5]]
```

**Example 3:**
```text
Input: candidates = [2], target = 1
Output: []
```

---

### Solution

At each index, branch into "use `candidates[index]` again" (stay at the same index, subtract from remaining) and "move past `candidates[index]`" (advance index, don't add it). This avoids duplicate combinations by never going back to earlier candidates.

```rust,ignore
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut current = Vec::new();

        Self::backtrack(&candidates, 0, target, &mut current, &mut result);

        result
    }

    pub fn backtrack(
    	nums: &[i32],
    	index: usize,
    	remaining: i32,
    	current: &mut Vec<i32>,
    	result: &mut Vec<Vec<i32>>,
    ) {
    	if remaining == 0 {
    		result.push(current.clone());
    		return;
    	}

    	if remaining < 0 || index == nums.len() {
    		return;
    	}

    	current.push(nums[index]);
    	Self::backtrack(nums, index, remaining - nums[index], current, result);

    	current.pop();
    	Self::backtrack(nums, index + 1, remaining, current, result);
    }
}
```
