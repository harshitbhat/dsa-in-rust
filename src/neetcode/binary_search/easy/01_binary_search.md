```rust,ignore
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
        	let mid: usize = (left + right) / 2;

        	if nums[mid] == target {
        		return mid as i32;
        	} else if nums[mid] < target {
        		left = mid + 1;
        	} else {
        		right = mid - 1;
        	}
        }

        -1
    }
}
```