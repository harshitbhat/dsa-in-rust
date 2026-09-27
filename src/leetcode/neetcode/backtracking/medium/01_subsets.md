# Subsets

[LeetCode #78](https://leetcode.com/problems/subsets/description/)

---

### Statement

Given an integer array `nums` of **unique** elements, return all possible subsets (the power set). The solution set must not contain duplicate subsets. Return the answer in any order.

**Constraints:**
- \\( 1 \le \text{nums.length} \le 10 \\)
- \\( -10 \le \text{nums}[i] \le 10 \\)
- All elements in `nums` are unique.

### Examples

**Example 1:**
```text
Input: nums = [1,2,3]
Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
```

**Example 2:**
```text
Input: nums = [0]
Output: [[],[0]]
```

---

### Solution

For each element, branch into "include it" and "skip it". When the index reaches the end of the array, the current accumulator is a complete subset — push a clone into the result.

```rust,ignore
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut current = Vec::new();

        Self::backtrack(&nums, 0, &mut current, &mut result);

        result
    }

    pub fn backtrack(
    	nums: &[i32],
    	index: usize,
    	current: &mut Vec<i32>,
    	result: &mut Vec<Vec<i32>>,
    ) {
    	// Base case: we've made a decision for every element
    	if index == nums.len() {
    		result.push(current.clone());
    		return;
    	}

    	// Branch 1: TAKE nums[index]
    	current.push(nums[index]);
    	Self::backtrack(nums, index + 1, current, result);

    	// Branch 2: SKIP nums[index]  ← this is the "backtrack" step
    	current.pop();
    	Self::backtrack(nums, index + 1, current, result);
    }
}
```
